use std::fmt::{Display, Error, Formatter};

use crate::check::constrain::constraint::expected::Expected;
use crate::common::delimit::comma_delm;

pub mod builder;
pub mod expected;
pub mod iterator;

#[derive(Clone, Debug)]
pub struct Constraint {
    pub is_flag: bool,
    pub is_sub:  bool,
    pub idents:  Vec<String>,
    pub msg:     String,
    pub parent:  Expected,
    pub child:   Expected
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let is_flag = if self.is_flag { "(flagged) " } else { "" };
        let is_sub = if self.is_sub { "(sub) " } else { "" };

        let msg = if self.msg.is_empty() { String::new() } else { format!("\"{}\" | ", self.msg) };
        let idents = if self.idents.is_empty() {
            String::new()
        } else {
            format!("(ids: {}) ", comma_delm(&self.idents))
        };
        let parent = if self.parent.is_expr() {
            format!("{}", self.parent)
        } else {
            format!("[{}]", self.parent)
        };
        let child = if self.child.is_expr() {
            format!("{}", self.child)
        } else {
            format!("[{}]", self.child)
        };
        let eq = if self.parent.is_ty() || self.child.is_ty() { ">=" } else { "=" };

        write!(f, "{}{}{}{}{} {} {}", msg, is_flag, is_sub, idents, parent, eq, child)
    }
}

impl Constraint {
    pub fn new(msg: &str, parent: &Expected, child: &Expected) -> Constraint {
        Constraint {
            parent:  parent.clone(),
            child:   child.clone(),
            msg:     String::from(msg),
            idents:  vec![],
            is_flag: false,
            is_sub:  false
        }
    }

    /// Flag constraint iff flagged is 0, else ignored.
    fn flag(&self) -> Constraint { Constraint { is_flag: true, ..self.clone() } }
}
