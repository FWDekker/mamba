use crate::parser::ast::{Node, AST};
use crate::type_checker::context::{function, Context};
use crate::type_checker::environment::infer_type::InferType;
use crate::type_checker::environment::state::State;
use crate::type_checker::environment::Environment;
use crate::type_checker::infer::{infer, InferResult};
use crate::type_checker::type_result::TypeErr;

pub fn infer_op(ast: &AST, env: &Environment, ctx: &Context, state: &State) -> InferResult {
    match &ast.node {
        Node::_Self => Ok((InferType::new(), env.clone())),
        Node::AddOp => Ok((InferType::new(), env.clone())),
        Node::SubOp => Ok((InferType::new(), env.clone())),
        Node::SqrtOp => Ok((InferType::new(), env.clone())),
        Node::MulOp => Ok((InferType::new(), env.clone())),
        Node::FDivOp => Ok((InferType::new(), env.clone())),
        Node::DivOp => Ok((InferType::new(), env.clone())),
        Node::PowOp => Ok((InferType::new(), env.clone())),
        Node::ModOp => Ok((InferType::new(), env.clone())),
        Node::EqOp => Ok((InferType::new(), env.clone())),
        Node::LeOp => Ok((InferType::new(), env.clone())),
        Node::GeOp => Ok((InferType::new(), env.clone())),

        Node::AddU { expr } => override_unary_op(expr, function::concrete::ADD, env, ctx, state),
        Node::SubU { expr } => override_unary_op(expr, function::concrete::SUB, env, ctx, state),
        Node::Sqrt { expr } => override_unary_op(expr, function::concrete::SQRT, env, ctx, state),

        Node::Add { left, right } =>
            override_op(left, right, function::concrete::ADD, env, ctx, state),
        Node::Sub { left, right } =>
            override_op(left, right, function::concrete::SUB, env, ctx, state),
        Node::Mul { left, right } =>
            override_op(left, right, function::concrete::MUL, env, ctx, state),
        Node::Div { left, right } =>
            override_op(left, right, function::concrete::DIV, env, ctx, state),
        Node::FDiv { left, right } =>
            override_op(left, right, function::concrete::FDIV, env, ctx, state),
        Node::Mod { left, right } =>
            override_op(left, right, function::concrete::MOD, env, ctx, state),
        Node::Pow { left, right } =>
            override_op(left, right, function::concrete::POW, env, ctx, state),
        Node::Le { left, right } =>
            override_op(left, right, function::concrete::LE, env, ctx, state),
        Node::Ge { left, right } =>
            override_op(left, right, function::concrete::GE, env, ctx, state),
        Node::Leq { left, right } =>
            override_op(left, right, function::concrete::LEQ, env, ctx, state),
        Node::Geq { left, right } =>
            override_op(left, right, function::concrete::GEQ, env, ctx, state),

        _ => Err(vec![TypeErr::new(&ast.pos, "Expected operation")])
    }
}

fn override_unary_op(
    expr: &Box<AST>,
    overrides: &str,
    env: &Environment,
    ctx: &Context,
    state: &State
) -> InferResult {
    let (infer_type, env) = infer(expr, env, ctx, state)?;
    let fun = infer_type.expr_ty(&expr.pos)?.fun(overrides, &vec![], state.safe, &expr.pos)?;
    match &fun.ty() {
        Some(fun_ty) => Ok((ctx.lookup(fun_ty, &expr.pos)?.add_raises(&infer_type), env)),
        None => Ok((InferType::new().add_raises(&infer_type), env))
    }
}

fn override_op(
    left: &Box<AST>,
    right: &Box<AST>,
    overrides: &str,
    env: &Environment,
    ctx: &Context,
    state: &State
) -> InferResult {
    let (left_ty, left_env) = infer(left, env, ctx, state)?;
    let (right_ty, right_env) = infer(right, &left_env, ctx, &state)?;
    let fun = left_ty.expr_ty(&left.pos)?.fun(
        overrides,
        &vec![right_ty.expr_ty(&right.pos)?],
        state.safe,
        &left.pos
    )?;

    match &fun.ty() {
        Some(fun_ty) => Ok((
            ctx.lookup(fun_ty, &left.pos.union(&right.pos))?
                .add_raises(&left_ty)
                .add_raises(&right_ty),
            right_env
        )),
        None => Ok((InferType::new().add_raises(&left_ty).add_raises(&right_ty), right_env))
    }
}
