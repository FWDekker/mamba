use crate::common::position::Position;
use crate::type_checker::context::field::generic::GenericField;
use crate::type_checker::context::type_name::actual::ActualTypeName;
use crate::type_checker::context::type_name::TypeName;
use crate::type_checker::type_result::{TypeErr, TypeResult};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub is_py_type: bool,
    pub name:       String,
    pub ty:         Option<TypeName>
}

impl Field {
    pub fn ty(&self) -> TypeResult<TypeName> {
        self.ty.clone().ok_or_else(|| vec![TypeErr::new_no_pos("Cannot infer type of field")])
    }
}

impl TryFrom<(&GenericField, &HashMap<String, ActualTypeName>, &Position)> for Field {
    type Error = Vec<TypeErr>;

    fn try_from(
        (field, generics, pos): (&GenericField, &HashMap<String, ActualTypeName>, &Position)
    ) -> Result<Self, Self::Error> {
        Ok(Field {
            is_py_type: field.is_py_type,
            name:       field.name.clone(),
            ty:         match &field.ty {
                Some(ty) => Some(ty.substitute(generics, pos)?),
                None => None
            }
        })
    }
}
