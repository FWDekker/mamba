use python_parser::ast::Expression;

use crate::check::context::function_arg::generic::GenericFunctionArg;
use crate::check::ty::name::TypeName;

pub const SELF: &str = "self";

impl From<(&String, &Option<Expression>, &Option<Expression>)> for GenericFunctionArg {
    fn from(
        (name, ty, default): (&String, &Option<Expression>, &Option<Expression>)
    ) -> GenericFunctionArg {
        GenericFunctionArg {
            is_py_type:  true,
            name:        name.clone(),
            has_default: default.is_some(),
            pos:         Default::default(),
            vararg:      false,
            mutable:     false,
            ty:          ty.clone().map(|e| TypeName::from(&e))
        }
    }
}
