use std::collections::HashMap;

use crate::common::position::Position;
use crate::type_checker::context::generic_type::GenericType;
use crate::type_checker::context::generic_type_name::GenericTypeName;
use crate::type_checker::environment::field::Field;
use crate::type_checker::environment::function::Function;
use crate::type_checker::environment::function_arg::FunctionArg;
use crate::type_checker::environment::type_name::TypeName;
use crate::type_checker::type_result::TypeErr;

#[derive(Debug, Clone)]
pub struct Type {
    pub name:      TypeName,
    pub concrete:  bool,
    pub args:      Vec<FunctionArg>,
    pub fields:    Vec<Field>,
    pub functions: Vec<Function>
}

impl Type {
    pub fn try_from(
        generic_type: &GenericType,
        generics: &HashMap<String, GenericTypeName>,
        pos: &Position
    ) -> Result<Self, TypeErr> {
        Ok(Type {
            name:      TypeName::Single {
                lit:      generic_type.name.clone(),
                generics: generic_type
                    .generics
                    .iter()
                    .map(|g| TypeName::Single { lit: g.name.clone(), generics: vec![] })
                    .collect()
            },
            concrete:  generic_type.concrete,
            args:      generic_type
                .args
                .iter()
                .map(|a| FunctionArg::try_from(a, generics, pos))
                .collect::<Result<_, _>>()?,
            fields:    generic_type
                .fields
                .iter()
                .map(|f| Field::try_from(f, generics, pos))
                .collect::<Result<_, _>>()?,
            functions: generic_type
                .functions
                .iter()
                .map(|f| Function::try_from(f, generics, pos))
                .collect::<Result<_, _>>()?
        })
    }
}
