use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;

use crate::parser::ast::{Node, AST};
use crate::type_checker::context::type_name::actual::ActualTypeName;
use crate::type_checker::context::type_name::TypeName;
use crate::type_checker::context::Context;
use crate::type_checker::environment::infer_type::InferType;
use crate::type_checker::environment::state::State;
use crate::type_checker::environment::Environment;
use crate::type_checker::infer::{infer, InferResult};
use crate::type_checker::type_result::TypeErr;
use crate::type_checker::util::comma_delimited;
use std::ops::Deref;

pub fn infer_error(ast: &AST, env: &Environment, ctx: &Context, state: &State) -> InferResult {
    match &ast.node {
        Node::Raise { error } => {
            let (ty, env) = infer(error, env, ctx, state)?;
            let actual_ty = ty.expr_ty(&error.pos)?.single(&error.pos)?.actual_ty();
            let set = HashSet::from_iter(vec![ActualTypeName::from(&actual_ty)].into_iter());
            Ok((InferType::new().union_raises(&set).add_raises(&ty), env))
        }
        Node::Raises { expr_or_stmt, errors } => {
            let (ty, env) = infer(expr_or_stmt, env, ctx, state)?;
            let errors = errors.iter().map(ActualTypeName::try_from).collect::<Result<_, _>>()?;
            if ty.raises.is_superset(&errors) {
                Ok((ty, env))
            } else {
                let new_set: HashSet<_> = errors.difference(&ty.raises).collect();
                let msg = format!(
                    "The following errors are mentioned but never raised: {}",
                    comma_delimited(new_set)
                );
                Err(vec![TypeErr::new(&expr_or_stmt.pos, &msg)])
            }
        }

        Node::Handle { expr_or_stmt, cases } => {
            let (cond_ty, mut env) = infer(expr_or_stmt, env, ctx, state)?;
            let state = state.handling(&cond_ty.raises.into_iter().collect());

            let mut ty: Option<InferType> = None;
            for case in cases {
                let (case_ty, new_env) = infer(case, &env, ctx, &state)?;
                env = new_env;
                ty = if let Some(ty) = ty {
                    Some(ty.union(&case_ty, &case.pos)?)
                } else {
                    Some(case_ty)
                };
            }

            match ty {
                Some(ty) => Ok((ty, env)),
                None => Err(vec![TypeErr::new(&ast.pos, "Match must have arms")])
            }
        }

        Node::Retry =>
            if !(state.in_handle) {
                Err(vec![TypeErr::new(&ast.pos, "Retry only possible in handle arm")])
            } else {
                Ok((InferType::new(), env.clone()))
            },

        Node::With { resource, _as, expr } => {
            let (resource_ty, mut env) = infer(resource, env, ctx, state)?;

            if let Some(_as) = _as {
                let (_as, mutable, type_name) = match &_as.node {
                    Node::IdType { id, _type, mutable } => match (&id.node, &_type) {
                        (Node::Id { lit }, Some(ty)) =>
                            (lit.clone(), mutable, Some(TypeName::try_from(ty.deref())?)),
                        (Node::Id { lit }, None) => (lit.clone(), mutable, None),
                        _ => return Err(vec![TypeErr::new(&_as.pos, "Expected identifier")])
                    },
                    _ => return Err(vec![TypeErr::new(&_as.pos, "Expected identifier")])
                };

                let expr_ty = resource_ty.expr_ty(&resource.pos)?;
                if let Some(type_name) = type_name {
                    if type_name != TypeName::from(&expr_ty) {
                        let msg =
                            format!("Expected {} but was {}", type_name, TypeName::from(&expr_ty));
                        return Err(vec![TypeErr::new(&resource.pos, &msg)]);
                    }
                }

                if let Node::Id { lit } = &resource.node {
                    env.remove(&lit);
                }
                env.insert(&_as, *mutable, &expr_ty);
            }

            infer(expr, &env, ctx, state)
        }

        _ => Err(vec![TypeErr::new(&ast.pos, "Expected error")])
    }
}
