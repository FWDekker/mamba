use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::check::context::arg::FunctionArg;
use crate::check::context::clss::generic::GenericClass;
use crate::check::context::field::Field;
use crate::check::context::function::Function;
use crate::check::context::name::Name;
use crate::check::result::{TypeErr, TypeResult};
use crate::check::ty::Type;
use crate::common::delimit::newline_delimited;
use crate::common::position::Position;

pub const INT_PRIMITIVE: &str = "Int";
pub const FLOAT_PRIMITIVE: &str = "Float";
pub const STRING_PRIMITIVE: &str = "String";
pub const BOOL_PRIMITIVE: &str = "Bool";
pub const ENUM_PRIMITIVE: &str = "Enum";
pub const COMPLEX_PRIMITIVE: &str = "Complex";

pub const RANGE: &str = "Range";
pub const SET: &str = "Set";
pub const LIST: &str = "List";

pub const NONE: &str = "undefined";
pub const EXCEPTION: &str = "Exception";

pub mod generic;
pub mod python;

/// Concrete type.
///
/// Has fields and functions defined within and from parents for easy access.
///
/// Parents are immediate parents.
///
/// Vector of fields signifies the fields of self, followed by fields of
/// consecutive parents and their parents. The same goes for functions.
#[derive(Debug, Clone, Eq)]
pub struct Class {
    pub is_py_type: bool,
    pub name:       Name,
    pub concrete:   bool,
    pub args:       Vec<FunctionArg>,
    pub fields:     Vec<HashSet<Field>>,
    pub parents:    HashSet<Name>,
    functions:      Vec<HashSet<Function>>
}

impl Hash for Class {
    fn hash<H: Hasher>(&self, state: &mut H) { self.name.hash(state) }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{}", self.name) }
}

impl TryFrom<(&GenericClass, &HashMap<String, Name>, &HashSet<GenericClass>, &Position)> for Class {
    type Error = Vec<TypeErr>;

    fn try_from(
        (generic, generics, types, pos): (
            &GenericClass,
            &HashMap<String, Name>,
            &HashSet<GenericClass>,
            &Position
        )
    ) -> Result<Self, Self::Error> {
        let self_name = generic.name.substitute(generics);
        let self_fields: HashSet<Field> = generic
            .fields
            .iter()
            .map(|field| Field::try_from((field, generics, pos)))
            .collect::<Result<_, _>>()?;
        let mut fields = vec![self_fields.clone()];
        let self_functions: HashSet<Function> = generic
            .functions
            .iter()
            .map(|fun| Function::try_from((fun, generics, pos)))
            .collect::<Result<_, _>>()?;
        let mut functions = vec![self_functions.clone()];

        let mut parents: HashSet<Type> = HashSet::new();
        for parent in &generic.parents {
            let name = Type::from(parent.name.as_str()).single(pos)?;
            let ty = types
                .iter()
                .find(|ty| ty.name == name)
                .ok_or_else(|| TypeErr::new(pos, &format!("Unknown parent type: {}", name)))?;

            let ty = Class::try_from((ty, generics, types, pos))?;
            fields.append(&mut ty.fields.clone());
            functions.append(&mut ty.functions.clone());
            parents.insert(Type::from(&name));
        }

        Ok(Class {
            is_py_type: generic.is_py_type,
            name: self_name,
            concrete: generic.concrete,
            args: generic
                .args
                .iter()
                .map(|a| FunctionArg::try_from((a, generics, pos)))
                .collect::<Result<_, _>>()?,
            parents,
            fields,
            functions
        })
    }
}

impl Class {
    pub fn field(&self, name: &str, pos: &Position) -> TypeResult<Field> {
        let field = self.fields.iter().flatten().find(|field| field.name.as_str() == name).cloned();
        field.ok_or_else(|| {
            vec![TypeErr::new(
                pos,
                &format!(
                    "Type {} does not define field \"{}\"{}{}",
                    self.name,
                    name,
                    if self.fields.is_empty() { "" } else { ", must be one of:\n" },
                    newline_delimited(&self.fields.iter().flatten().collect::<Vec<&Field>>())
                )
            )]
        })
    }

    pub fn function(&self, fun_name: &Name, pos: &Position) -> TypeResult<Function> {
        self.functions
            .iter()
            .find_map(|functions| {
                functions.iter().find_map(|function| {
                    if Type::from(&function.name) == fun_name.clone() {
                        Some(function.clone())
                    } else {
                        None
                    }
                })
            })
            .ok_or_else(|| {
                vec![TypeErr::new(
                    pos,
                    &format!(
                        "Type {} does not define function \"{}\"{}{}",
                        self,
                        fun_name,
                        if self.functions.is_empty() { "" } else { ", must be one of: \n" },
                        newline_delimited(
                            &self.functions.iter().flatten().collect::<Vec<&Function>>()
                        )
                    )
                )]
            })
    }
}

pub fn concrete_to_python(name: &str) -> String {
    match name {
        INT_PRIMITIVE => String::from(python::INT_PRIMITIVE),
        FLOAT_PRIMITIVE => String::from(python::FLOAT_PRIMITIVE),
        STRING_PRIMITIVE => String::from(python::STRING_PRIMITIVE),
        BOOL_PRIMITIVE => String::from(python::BOOL_PRIMITIVE),
        ENUM_PRIMITIVE => String::from(python::ENUM_PRIMITIVE),
        COMPLEX_PRIMITIVE => String::from(python::COMPLEX_PRIMITIVE),

        RANGE => String::from(python::RANGE),
        SET => String::from(python::SET),
        LIST => String::from(python::LIST),

        NONE => String::from(python::NONE),
        other => String::from(other)
    }
}
