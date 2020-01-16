use crate::parser::ast::{Node, AST};
use crate::type_checker::constraints::cons::Expect::{Expression, ExpressionAny, Implements,
                                                     Truthy, Type};
use crate::type_checker::constraints::cons::{Constraints, Expected};
use crate::type_checker::constraints::generate::collection::constrain_collection;
use crate::type_checker::constraints::generate::generate;
use crate::type_checker::constraints::Constrained;
use crate::type_checker::context::function::concrete::*;
use crate::type_checker::context::ty::concrete::{FLOAT_PRIMITIVE, INT_PRIMITIVE, STRING_PRIMITIVE};
use crate::type_checker::context::{ty, Context};
use crate::type_checker::environment::Environment;
use crate::type_checker::type_name::TypeName;
use crate::type_checker::type_result::TypeErr;

pub fn gen_op(ast: &AST, env: &Environment, ctx: &Context, constr: &Constraints) -> Constrained {
    match &ast.node {
        Node::In { left, right } => constrain_collection(right, left, env, ctx, constr),
        Node::Range { from, to, step: Some(step), .. } => {
            let type_name = TypeName::from(ty::concrete::INT_PRIMITIVE);
            let l_exp = Expected::new(&from.pos, &Expression { ast: *from.clone() });
            let constr = constr
                .add(&l_exp, &Expected::new(&from.pos, &Type { type_name: type_name.clone() }));

            let l_exp = Expected::new(&to.pos, &Expression { ast: *to.clone() });
            let constr =
                constr.add(&l_exp, &Expected::new(&to.pos, &Type { type_name: type_name.clone() }));

            let l_exp = Expected::new(&step.pos, &Expression { ast: *step.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&step.pos, &Type { type_name }));

            let (constr, env) = generate(from, env, ctx, &constr)?;
            let (constr, env) = generate(to, &env, ctx, &constr)?;
            generate(step, &env, ctx, &constr)
        }
        Node::Range { from, to, .. } => {
            let type_name = TypeName::from(ty::concrete::INT_PRIMITIVE);
            let l_exp = Expected::new(&from.pos, &Expression { ast: *from.clone() });
            let constr = constr
                .add(&l_exp, &Expected::new(&from.pos, &Type { type_name: type_name.clone() }));

            let l_exp = Expected::new(&to.pos, &Expression { ast: *to.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&to.pos, &Type { type_name }));

            let (constr, env) = generate(from, env, ctx, &constr)?;
            generate(to, &env, ctx, &constr)
        }

        Node::Real { .. } => primitive(ast, FLOAT_PRIMITIVE, env, constr),
        Node::Int { .. } => primitive(ast, INT_PRIMITIVE, env, constr),
        Node::ENum { .. } => primitive(ast, INT_PRIMITIVE, env, constr),
        Node::Str { .. } => primitive(ast, STRING_PRIMITIVE, env, constr),
        Node::Bool { .. } => {
            let left = Expected::new(&ast.pos, &Expression { ast: ast.clone() });
            let constr = constr.add(&left, &Expected::new(&ast.pos, &Truthy));
            Ok((constr, env.clone()))
        }

        Node::Add { left, right } => implements(ADD, left, right, env, ctx, constr),
        Node::Sub { left, right } => implements(SUB, left, right, env, ctx, constr),
        Node::Mul { left, right } => implements(MUL, left, right, env, ctx, constr),
        Node::Div { left, right } => implements(DIV, left, right, env, ctx, constr),
        Node::FDiv { left, right } => implements(FDIV, left, right, env, ctx, constr),
        Node::Pow { left, right } => implements(POW, left, right, env, ctx, constr),
        Node::Le { left, right } => implements(LE, left, right, env, ctx, constr),
        Node::Ge { left, right } => implements(GE, left, right, env, ctx, constr),
        Node::Leq { left, right } => implements(LEQ, left, right, env, ctx, constr),
        Node::Geq { left, right } => implements(GEQ, left, right, env, ctx, constr),
        Node::Eq { left, right } => implements(EQ, left, right, env, ctx, constr),
        Node::Mod { left, right } => implements(MOD, left, right, env, ctx, constr),
        Node::Neq { left, right } => implements(NEQ, left, right, env, ctx, constr),
        Node::AddU { expr } | Node::SubU { expr } => generate(expr, env, ctx, &constr),
        Node::Sqrt { expr } => {
            let left = Expected::new(&expr.pos, &Expression { ast: *expr.clone() });
            let right = Expected::new(&expr.pos, &Implements {
                type_name: TypeName::from(SQRT),
                args:      vec![Expression { ast: *expr.clone() }]
            });
            let constr = constr.add(&left, &right);
            generate(expr, &env, ctx, &constr)
        }

        Node::BOneCmpl { expr } => {
            let left = Expected::new(&expr.pos, &Expression { ast: *expr.clone() });
            let constr = constr.add(&left, &Expected::new(&expr.pos, &ExpressionAny));
            generate(expr, &env, ctx, &constr)
        }
        Node::BAnd { left, right } | Node::BOr { left, right } | Node::BXOr { left, right } => {
            let l_exp = Expected::new(&left.pos, &Expression { ast: *left.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&left.pos, &ExpressionAny));

            let l_exp = Expected::new(&right.pos, &Expression { ast: *right.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&right.pos, &ExpressionAny));
            let (constr, env) = generate(right, env, ctx, &constr)?;
            generate(left, &env, ctx, &constr)
        }
        Node::BLShift { left, right } | Node::BRShift { left, right } => {
            let l_exp = Expected::new(&left.pos, &Expression { ast: *left.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&right.pos, &ExpressionAny));

            let type_name = TypeName::from(ty::concrete::INT_PRIMITIVE);
            let l_exp = Expected::new(&right.pos, &Expression { ast: *right.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&right.pos, &Type { type_name }));

            let (constr, env) = generate(right, env, ctx, &constr)?;
            generate(left, &env, ctx, &constr)
        }

        Node::Is { left, right } | Node::IsN { left, right } => {
            let (constr, env) = generate(right, env, ctx, constr)?;
            generate(left, &env, ctx, &constr)
        }
        Node::IsA { left, right } | Node::IsNA { left, right } => {
            let (constr, env) = generate(right, env, ctx, constr)?;
            generate(left, &env, ctx, &constr)
        }

        Node::Not { expr } => {
            let left = Expected::new(&expr.pos, &Expression { ast: *expr.clone() });
            let constr = constr.add(&left, &Expected::new(&expr.pos, &Truthy));
            generate(expr, env, ctx, &constr)
        }
        Node::And { left, right } | Node::Or { left, right } => {
            let l_exp = Expected::new(&left.pos, &Expression { ast: *left.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&left.pos, &Truthy));

            let l_exp = Expected::new(&right.pos, &Expression { ast: *right.clone() });
            let constr = constr.add(&l_exp, &Expected::new(&right.pos, &Truthy));

            let (constr, env) = generate(left, env, ctx, &constr)?;
            generate(right, &env, &ctx, &constr)
        }

        _ => Err(vec![TypeErr::new(&ast.pos, "Was expecting operation or primitive")])
    }
}

fn primitive(ast: &AST, ty: &str, env: &Environment, constr: &Constraints) -> Constrained {
    // Do not recursively check ast
    let type_name = TypeName::from(ty);
    let left = Expected::new(&ast.pos, &Expression { ast: ast.clone() });
    let constr = constr.add(&left, &Expected::new(&ast.pos, &Type { type_name }));
    Ok((constr, env.clone()))
}

fn implements(
    fun: &str,
    left: &AST,
    right: &AST,
    env: &Environment,
    ctx: &Context,
    constr: &Constraints
) -> Constrained {
    let l_exp = Expected::new(&left.pos, &Expression { ast: left.clone() });
    let constr = constr.add(&l_exp, &Expected::new(&right.pos, &Expression { ast: right.clone() }));

    let l_exp = Expected::new(&left.pos, &Expression { ast: left.clone() });
    let r_exp = Expected::new(&left.pos, &Implements {
        type_name: TypeName::from(fun),
        args:      vec![Expression { ast: left.clone() }, Expression { ast: right.clone() }]
    });

    let constr = constr.add(&l_exp, &r_exp);
    let (constr, env) = generate(left, env, ctx, &constr)?;
    generate(right, &env, ctx, &constr)
}
