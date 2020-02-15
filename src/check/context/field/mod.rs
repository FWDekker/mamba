use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::check::context::field::generic::GenericField;
use crate::check::context::name::Name;
use crate::check::result::TypeErr;
use crate::common::position::Position;
use std::fmt;

pub mod generic;
pub mod python;

/// A Field, which may either be top-level, or optionally within a class.
///
/// May have a type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    pub is_py_type: bool,
    pub name:       String,
    pub private:    bool,
    pub mutable:    bool,
    pub in_class:   Option<Name>,
    pub ty:         Option<Name>
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            &self.name,
            if let Some(ty) = &self.ty { format!(": {}", ty) } else { String::new() }
        )
    }
}

impl TryFrom<(&GenericField, &HashMap<String, Name>, &Position)> for Field {
    type Error = Vec<TypeErr>;

    fn try_from(
        (field, generics, pos): (&GenericField, &HashMap<String, Name>, &Position)
    ) -> Result<Self, Self::Error> {
        Ok(Field {
            is_py_type: field.is_py_type,
            name:       field.name.clone(),
            private:    field.private,
            mutable:    field.mutable,
            in_class:   match &field.in_class {
                Some(in_class) => Some(in_class.substitute(generics, pos)?),
                None => None
            },
            ty:         match &field.type_name {
                Some(ty) => Some(ty.substitute(generics, pos)?),
                None => None
            }
        })
    }
}
