use itertools::{EitherOrBoth, Itertools};

use crate::common::position::Position;
use crate::parser::ast::AST;
use crate::type_checker::constraints::cons::Expect::*;
use crate::type_checker::type_name::TypeName;

#[derive(Clone, Debug)]
pub struct Constraints {
    pub constraints: Vec<Constraint>
}

impl Constraints {
    pub fn new() -> Constraints { Constraints { constraints: vec![] } }

    pub fn append(&mut self, constraints: &Constraints) -> Constraints {
        let mut new_constr = self.constraints.clone();
        new_constr.append(&mut constraints.constraints.clone());
        Constraints { constraints: new_constr }
    }

    pub fn add_constraint(&self, constraint: &Constraint) -> Constraints {
        let mut constraints = self.constraints.clone();
        constraints.push(constraint.clone());
        Constraints { constraints }
    }

    pub fn push(&mut self, left: &Expected, right: &Expected) {
        self.constraints.push(Constraint(left.clone(), right.clone()))
    }

    pub fn add(&self, left: &Expected, right: &Expected) -> Constraints {
        let mut constraints = self.constraints.clone();
        constraints.push(Constraint(left.clone(), right.clone()));
        Constraints { constraints }
    }
}

impl From<&Constraint> for Constraints {
    fn from(constraint: &Constraint) -> Self {
        Constraints { constraints: vec![constraint.clone()] }
    }
}

#[derive(Clone, Debug)]
pub struct Constraint(pub Expected, pub Expected);

#[derive(Clone, Debug, Eq)]
pub struct Expected {
    pub pos:    Position,
    pub expect: Expect
}

impl PartialEq for Expected {
    fn eq(&self, other: &Self) -> bool { self.expect == other.expect }
}

impl Expected {
    pub fn new(pos: &Position, expect: &Expect) -> Expected {
        Expected { pos: pos.clone(), expect: expect.clone() }
    }
}

// TODO rework HasField

#[derive(Clone, Debug, Eq)]
pub enum Expect {
    Nullable { expect: Box<Expect> },
    Mutable { expect: Box<Expect> },
    Expression { ast: AST },
    ExpressionAny,

    Collection { ty: Box<Expect> },
    Truthy,

    RaisesAny,
    Raises { type_name: TypeName },

    Implements { type_name: TypeName, args: Vec<Expected> },
    Function { name: TypeName, args: Vec<Expected> },
    HasFunction { name: TypeName, args: Vec<Expected> },
    HasField { name: String },

    Type { type_name: TypeName }
}

impl PartialEq for Expect {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Nullable { expect: l }, Nullable { expect: r })
            | (Mutable { expect: l }, Mutable { expect: r }) => l == r,
            (Collection { ty: l }, Collection { ty: r }) => l == r,
            (HasField { name: l }, HasField { name: r }) => l == r,
            (Raises { type_name: l }, Raises { type_name: r })
            | (Type { type_name: l }, Type { type_name: r }) => l == r,

            (Implements { type_name: l, args: la }, Implements { type_name: r, args: ra })
            | (Function { name: l, args: la }, Function { name: r, args: ra })
            | (HasFunction { name: l, args: la }, HasFunction { name: r, args: ra }) =>
                l == r
                    && la.iter().zip_longest(ra.iter()).all(|pair| {
                        if let EitherOrBoth::Both(left, right) = pair {
                            left == right
                        } else {
                            false
                        }
                    }),
            (Expression { ast: l }, Expression { ast: r }) => l.equal_structure(r),
            (Truthy, Truthy) | (RaisesAny, RaisesAny) | (ExpressionAny, ExpressionAny) => true,
            _ => false
        }
    }
}
