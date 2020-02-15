use std::ops::Deref;

use python_parser::ast::{Expression, SetItem, Subscript};

use crate::check::context::clss::python::python_to_concrete;
use crate::check::context::name::{Name, NameUnion};

impl From<&Expression> for NameUnion {
    fn from(value: &Expression) -> Self {
        match value {
            Expression::Name(_) => NameUnion::from(&Name::from(value)),
            Expression::TupleLiteral(_) => NameUnion::from(&Name::from(value)),
            Expression::Subscript(id, exprs) =>
                if id == Expression::Name(String::from("Union")) {
                    NameUnion::new(&exprs.iter().map(|e| to_ty_name(e)).collect())
                } else {
                    NameUnion::from(&Name::from(value))
                },
            _ => NameUnion::from(&Name::empty())
        }
    }
}

impl From<&Expression> for Name {
    fn from(value: &Expression) -> Name {
        match value {
            Expression::Name(id) => Name::from(python_to_concrete(&id.clone()).as_str()),
            Expression::TupleLiteral(items) => {
                let expressions = items.iter().filter_map(|setitem| match setitem {
                    SetItem::Star(_) => None,
                    SetItem::Unique(expr) => Some(expr)
                });
                Name::Tuple(expressions.map(|expr| Name::from(expr)).collect())
            }
            Expression::Subscript(id, exprs) => {
                let lit = match &id.deref() {
                    Expression::Name(name) => name.clone(),
                    _ => String::new()
                };

                // Union not expected
                if lit == String::from("Union") {
                    Name::empty()
                } else {
                    let generics: Vec<_> = exprs.iter().map(|e| to_ty_name(e)).collect();
                    Name::new(&lit, &generics)
                }
            }
            _ => Name::empty()
        }
    }
}

fn to_ty_name(sub_script: &Subscript) -> Name {
    match sub_script {
        Subscript::Simple(expr) => Name::from(expr),
        _ => Name::empty()
    }
}
