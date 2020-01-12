use crate::parser::ast::{Node, AST};
use crate::type_checker::constraints::cons::Constraints;
use crate::type_checker::constraints::cons::Expect::{Expression, Raises};
use crate::type_checker::constraints::generate::generate;
use crate::type_checker::constraints::Constrained;
use crate::type_checker::context::Context;
use crate::type_checker::environment::Environment;
use crate::type_checker::type_name::TypeName;
use crate::type_checker::type_result::TypeErr;
use std::convert::TryFrom;

pub fn gen_resources(
    ast: &AST,
    env: &Environment,
    ctx: &Context,
    constr: &Constraints
) -> Constrained {
    match &ast.node {
        Node::Raises { expr_or_stmt, errors } => {
            let (constr, env) = constrain_raises(expr_or_stmt, errors, env, ctx, constr)?;
            generate(expr_or_stmt, &env, ctx, &constr)
        }
        Node::With { resource, alias: Some(alias), expr } => {
            let constr = constr
                .add(&Expression { ast: *resource.clone() }, &Expression { ast: *alias.clone() });
            let (constr, env) = generate(resource, env, ctx, &constr)?;
            let (constr, env) = generate(alias, &env, ctx, &constr)?;
            generate(expr, &env, ctx, &constr)
        }
        Node::With { resource, expr, .. } => {
            let (constr, env) = generate(resource, env, ctx, &constr)?;
            generate(expr, &env, ctx, &constr)
        }

        _ => Err(vec![TypeErr::new(&ast.pos, "Expected resources")])
    }
}

pub fn constrain_raises(
    expr: &AST,
    errors: &Vec<AST>,
    env: &Environment,
    ctx: &Context,
    constr: &Constraints
) -> Constrained {
    let mut res = (constr.clone(), env.clone());
    for error in errors {
        let type_name = TypeName::try_from(error)?;
        res.0 = res.0.add(&Expression { ast: expr.clone() }, &Raises { type_name });
        res = generate(error, &res.1, ctx, &res.0)?;
    }
    Ok(res)
}
