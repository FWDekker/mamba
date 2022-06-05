use std::fmt::{Display, Error, Formatter};

use crate::check::constrain::constraint::expected::Expect::{Access, Function, Type};
use crate::check::constrain::constraint::expected::Expected;
use crate::check::context::{clss, function};
use crate::check::name::Name;
use crate::check::name::stringname::StringName;

pub mod builder;
pub mod expected;
pub mod iterator;

#[derive(Clone, Debug)]
pub struct Constraint {
    pub is_flag: bool,
    pub is_sub: bool,
    pub msg: String,
    pub left: Expected,
    pub right: Expected,
    pub superset: ConstrVariant,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstrVariant {
    Left,
    Right,
    Either,
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let superset = match &self.superset {
            ConstrVariant::Left => "{left}  ",
            ConstrVariant::Right => "{right}  ",
            _ => ""
        };

        write!(f, "{}{} == {}", superset, self.left, self.right)
    }
}

impl Constraint {
    /// Create new constraint.
    ///
    /// By default, the left side is assumed to be the superset of the right side.
    pub fn new(msg: &str, parent: &Expected, child: &Expected) -> Constraint {
        Constraint::new_variant(msg, parent, child, &ConstrVariant::Left)
    }

    pub fn new_variant(msg: &str, parent: &Expected, child: &Expected, superset: &ConstrVariant)
                       -> Constraint {
        Constraint {
            left: parent.clone(),
            right: child.clone(),
            msg: String::from(msg),
            is_flag: false,
            is_sub: false,
            superset: superset.clone(),
        }
    }

    /// Flag constraint iff flagged is 0, else ignored.
    fn flag(&self) -> Constraint { Constraint { is_flag: true, ..self.clone() } }

    pub fn stringy(msg: &str, expected: &Expected) -> Constraint {
        let string =
            Expected::new(expected.pos, &Type { name: Name::from(clss::STRING) });
        let access = Access {
            entity: Box::from(expected.clone()),
            name: Box::new(Expected::new(expected.pos, &Function {
                name: StringName::from(function::STR),
                args: vec![expected.clone()],
            })),
        };

        Constraint::new(msg, &string, &Expected::new(expected.pos, &access))
    }

    pub fn truthy(msg: &str, expected: &Expected) -> Constraint {
        let bool =
            Expected::new(expected.pos, &Type { name: Name::from(clss::BOOL) });
        let access = Access {
            entity: Box::from(expected.clone()),
            name: Box::new(Expected::new(expected.pos, &Function {
                name: StringName::from(function::TRUTHY),
                args: vec![expected.clone()],
            })),
        };

        Constraint::new(msg, &bool, &Expected::new(expected.pos, &access))
    }
}
