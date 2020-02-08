use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ops::Deref;

use crate::common::position::Position;
use crate::type_checker::checker_result::{TypeErr, TypeResult};
use crate::type_checker::context::field::concrete::Field;
use crate::type_checker::context::function::concrete::Function;
use crate::type_checker::context::function_arg::concrete::FunctionArg;
use crate::type_checker::context::ty::concrete::Type;
use crate::type_checker::context::{function, Context};
use crate::type_checker::ty::expression::ExpressionType;
use crate::type_checker::ty_name::TypeName;
use crate::type_checker::util::comma_delimited;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum ActualType {
    Single { ty: Type },
    Tuple { types: Vec<ExpressionType> },
    AnonFun { args: Vec<ExpressionType>, ret_ty: Box<ExpressionType> }
}

impl Display for ActualType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            ActualType::Single { ty } => write!(f, "{}", ty),
            ActualType::Tuple { types } => write!(f, "({})", comma_delimited(types)),
            ActualType::AnonFun { args, ret_ty } =>
                write!(f, "({}) -> {}", comma_delimited(args), ret_ty),
        }
    }
}

impl ActualType {
    /// Has parent if:
    /// - Self name is `type_name`
    /// - Immediate parent is `type_name`
    /// - One of parents has `type_name` as parent
    pub fn has_parent(
        &self,
        type_name: &TypeName,
        checked: &HashSet<TypeName>,
        ctx: &Context,
        pos: &Position
    ) -> TypeResult<bool> {
        match &self {
            ActualType::Single { ty } => {
                let immediate_parents = ty.parents.clone();
                Ok(TypeName::from(&ty.name) == type_name.clone()
                    || immediate_parents.contains(type_name)
                    || {
                        let parent_tys: Vec<ExpressionType> = immediate_parents
                            .iter()
                            .map(|p| ctx.lookup(p, pos))
                            .collect::<Result<_, _>>()?;
                        let bools: Vec<bool> = parent_tys
                            .iter()
                            .map(|p_ty| p_ty.has_parent_checked(type_name, checked, ctx, pos))
                            .collect::<Result<_, _>>()?;
                        bools.iter().any(|b| *b)
                    })
            }
            ActualType::Tuple { .. } =>
                Err(vec![TypeErr::new(pos, &format!("A tuple {} does not have parents", &self))]),
            ActualType::AnonFun { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("A anonymous function {} does not have parents", &self)
            )])
        }
    }

    pub fn fields(&self, pos: &Position) -> TypeResult<HashSet<Field>> {
        match &self {
            ActualType::Single { ty } => Ok(ty.fields.clone()),
            ActualType::Tuple { .. } =>
                Err(vec![TypeErr::new(pos, &format!("A tuple {} does not have fields", &self))]),
            ActualType::AnonFun { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("A anonymous function {} does not have fields", &self)
            )])
        }
    }

    pub fn field(&self, field: &str, pos: &Position) -> TypeResult<Field> {
        match &self {
            ActualType::Single { ty } => ty.field(field, pos),
            ActualType::Tuple { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("A tuple {} cannot have field: {}", &self, field)
            )]),
            ActualType::AnonFun { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("A anonymous function {} cannot have field: {}", &self, field)
            )])
        }
    }

    pub fn anon_fun_params(&self, pos: &Position) -> TypeResult<(Vec<TypeName>, TypeName)> {
        match &self {
            ActualType::AnonFun { args, ret_ty } =>
                Ok((args.iter().map(TypeName::from).collect(), TypeName::from(ret_ty.deref()))),
            ActualType::Single { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("Expected an anonymous function but was {}", &self)
            )]),
            ActualType::Tuple { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("Expected an anonymous function but was a tuple {}", &self)
            )])
        }
    }

    /// Check if a type implements a a function.
    ///
    /// [STR](function::concrete::STR) is a special case.
    /// For tuples, we proceed to check if every item in the tuple implements
    /// [STR](function::concrete::STR).
    pub fn function(&self, name: &TypeName, pos: &Position) -> TypeResult<HashSet<Function>> {
        match &self {
            ActualType::Single { ty } =>
                Ok(HashSet::from_iter(vec![ty.function(name, pos)?].into_iter())),
            ActualType::Tuple { types } =>
                if name == &TypeName::from(function::concrete::STR) {
                    let ty: Vec<HashSet<Function>> = types
                        .into_iter()
                        .map(|ty| ty.function(name, pos))
                        .collect::<Result<_, _>>()?;
                    Ok(ty.into_iter().flatten().collect())
                } else {
                    Err(vec![TypeErr::new(
                        pos,
                        &format!("A tuple {} cannot define function {}", &self, name)
                    )])
                },
            ActualType::AnonFun { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("A anonymous function {} cannot define function {}", &self, name)
            )])
        }
    }

    pub fn constructor_args(&self, pos: &Position) -> TypeResult<Vec<FunctionArg>> {
        match &self {
            ActualType::Single { ty } => Ok(ty.args.clone()),
            ActualType::Tuple { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("A tuple {} does not have constructor arguments", &self)
            )]),
            ActualType::AnonFun { .. } => Err(vec![TypeErr::new(
                pos,
                &format!("A anonymous function {} does not have constructor arguments", &self)
            )])
        }
    }
}
