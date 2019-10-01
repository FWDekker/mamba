use core::fmt;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::common::position::Position;
use crate::type_checker::context::concrete::type_name::actual::ActualTypeName;
use crate::type_checker::context::generic::type_name::{GenericActualTypeName, GenericTypeName};
use crate::type_checker::type_result::{TypeErr, TypeResult};

pub mod actual;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeName {
    Single { ty: ActualTypeName },
    Union { union: HashSet<ActualTypeName> }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TypeName::Single { ty } => write!(f, "{}", ty),
            TypeName::Union { union } => write!(f, "{{{:#?}}}", union)
        }
    }
}

impl TryFrom<(&GenericTypeName, &HashMap<String, GenericActualTypeName>, &Position)> for TypeName {
    type Error = Vec<TypeErr>;

    fn try_from(
        (gen_type_name, generics, pos): (
            &GenericTypeName,
            &HashMap<String, GenericActualTypeName>,
            &Position
        )
    ) -> TypeResult<Self> {
        match gen_type_name {
            GenericTypeName::Single { ty } =>
                Ok(TypeName::Single { ty: ActualTypeName::try_from((ty, generics, pos))? }),
            GenericTypeName::Union { union } => {
                let (union, errs) = union
                    .iter()
                    .map(|ty| ActualTypeName::try_from((ty, generics, pos)))
                    .partition(Result::is_ok);

                if errs.is_empty() {
                    Ok(TypeName::Union {
                        union: union.into_iter().map(Result::unwrap_err).collect()
                    })
                } else {
                    Err(errs.into_iter().map(Result::unwrap_err).collect())
                }
            }
        }
    }
}

impl TypeName {
    pub fn new(lit: &str, generics: &[ActualTypeName]) -> TypeName {
        TypeName::Single {
            ty: ActualTypeName::Single {
                lit:      String::from(lit),
                generics: Vec::from(generics)
            }
        }
    }
}
