use crate::parser::ast::{Node, AST};
use crate::type_checker::context::ty::concrete;
use crate::type_checker::context::type_name::TypeName;
use crate::type_checker::context::Context;
use crate::type_checker::environment::state::State;
use crate::type_checker::environment::Environment;
use crate::type_checker::infer::{infer, InferResult};
use crate::type_checker::type_result::TypeErr;

pub fn infer_bitwise_op(ast: &AST, env: &Environment, ctx: &Context, state: &State) -> InferResult {
    match &ast.node {
        Node::BAnd { left, right }
        | Node::BOr { left, right }
        | Node::BXOr { left, right }
        | Node::BLShift { left, right }
        | Node::BRShift { left, right } => {
            let (left_ty, env) = infer(left, env, ctx, state)?;
            let (right_ty, env) = infer(right, &env, ctx, &state)?;
            left_ty.expr_ty(&ast.pos)?;
            right_ty.expr_ty(&ast.pos)?;
            Ok((
                ctx.lookup(&TypeName::new(concrete::INT_PRIMITIVE, &vec![]), &ast.pos)?
                    .add_raises(&left_ty)
                    .add_raises(&right_ty),
                env.clone()
            ))
        }
        Node::BOneCmpl { expr } => {
            let (infer_ty, env) = infer(expr, env, ctx, state)?;
            infer_ty.expr_ty(&ast.pos)?;
            Ok((
                ctx.lookup(&TypeName::new(concrete::INT_PRIMITIVE, &vec![]), &ast.pos)?
                    .add_raises(&infer_ty),
                env.clone()
            ))
        }
        _ => Err(vec![TypeErr::new(&ast.pos, "Expected bitwise operation")])
    }
}
