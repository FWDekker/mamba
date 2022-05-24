use std::collections::VecDeque;

use crate::check::constrain::constraint::Constraint;
use crate::check::constrain::constraint::expected::Expected;
use crate::check::name::stringname::StringName;
use crate::check::result::{TypeErr, TypeResult};

#[derive(Clone, Debug)]
pub struct Constraints {
    pub in_class: Vec<StringName>,
    constraints: VecDeque<Constraint>,
}

impl From<&(Vec<StringName>, Vec<Constraint>)> for Constraints {
    fn from((in_class, constraints): &(Vec<StringName>, Vec<Constraint>)) -> Self {
        let constraints = VecDeque::from(constraints.clone());
        Constraints { in_class: in_class.clone(), constraints }
    }
}

impl Constraints {
    pub fn new(in_class: &[StringName]) -> Constraints {
        Constraints { in_class: Vec::from(in_class), constraints: VecDeque::new() }
    }

    pub fn len(&self) -> usize { self.constraints.len() }

    pub fn pop_constr(&mut self) -> Option<Constraint> { self.constraints.pop_front() }

    /// Push constraint at front so that it will be analysed next.
    ///
    /// Only used during unification stage.
    /// Marks constraint as generated.
    pub fn push(&mut self, msg: &str, left: &Expected, right: &Expected) {
        let constraint = Constraint::new(msg, left, right);
        trace!("{:width$}[gen {}] {}", "", msg, constraint, width = 17);
        self.constraints.push_front(constraint)
    }

    /// Append in_class and constraints of constraints to self
    pub fn append(&mut self, constraints: &mut Constraints) {
        self.in_class.append(&mut constraints.in_class);
        self.constraints.append(&mut constraints.constraints)
    }

    pub fn push_constr(&mut self, constr: &Constraint) {
        self.constraints.push_back(constr.clone())
    }

    pub fn reinsert(&mut self, constraint: &Constraint) -> TypeResult<()> {
        if constraint.is_flag {
            // Can only reinsert constraint once
            let msg = format!(
                "Cannot infer type. Expected a {}, was {}",
                &constraint.left.expect, &constraint.right.expect
            );
            return Err(vec![TypeErr::new(&constraint.left.pos, &msg)]);
        }

        self.constraints.push_back(constraint.flag());
        Ok(())
    }
}

impl Default for Constraints {
    fn default() -> Self { Constraints::new(&[]) }
}
