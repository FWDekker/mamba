use mamba::core::construct::Core;
use mamba::desugar::desugar;
use mamba::parser::ast::ASTNode;
use mamba::parser::ast::ASTNodePos;

#[test]
fn if_verify() {
    let cond = to_pos!(ASTNode::Id { lit: String::from("cond") });
    let then = to_pos!(ASTNode::Id { lit: String::from("then") });
    let if_stmt = to_pos!(ASTNode::IfElse { cond, then, _else: None });

    let (core_cond, core_then) = match desugar(&if_stmt) {
        Core::If { cond, then } => (cond, then),
        other => panic!("Expected reassign but was {:?}", other)
    };

    assert_eq!(*core_cond, Core::Id { lit: String::from("cond") });
    assert_eq!(*core_then, Core::Id { lit: String::from("then") });
}

#[test]
fn if_else_verify() {
    let cond = to_pos!(ASTNode::Id { lit: String::from("cond") });
    let then = to_pos!(ASTNode::Id { lit: String::from("then") });
    let _else = to_pos!(ASTNode::Id { lit: String::from("else") });
    let if_stmt = to_pos!(ASTNode::IfElse { cond, then, _else: Some(_else) });

    let (core_cond, core_then, core_else) = match desugar(&if_stmt) {
        Core::IfElse { cond, then, _else } => (cond, then, _else),
        other => panic!("Expected reassign but was {:?}", other)
    };

    assert_eq!(*core_cond, Core::Id { lit: String::from("cond") });
    assert_eq!(*core_then, Core::Id { lit: String::from("then") });
    assert_eq!(*core_else, Core::Id { lit: String::from("else") });
}

#[test]
fn while_verify() {
    let cond = to_pos!(ASTNode::Id { lit: String::from("cond") });
    let body = to_pos!(ASTNode::ENum { num: String::from("num"), exp: String::from("") });
    let while_stmt = to_pos!(ASTNode::While { cond, body });

    let (core_cond, core_body) = match desugar(&while_stmt) {
        Core::While { cond, body } => (cond, body),
        other => panic!("Expected reassign but was {:?}", other)
    };

    assert_eq!(*core_cond, Core::Id { lit: String::from("cond") });
    assert_eq!(*core_body, Core::ENum { num: String::from("num"), exp: String::from("0") });
}

#[test]
fn for_verify() {
    let expr = to_pos!(ASTNode::Id { lit: String::from("expr_1") });
    let body = to_pos!(ASTNode::Id { lit: String::from("body") });
    let for_stmt = to_pos!(ASTNode::For { expr, body });

    let (core_expr, core_body) = match desugar(&for_stmt) {
        Core::For { expr, body } => (expr, body),
        other => panic!("Expected for but was {:?}", other)
    };

    assert_eq!(*core_expr, Core::Id { lit: String::from("expr_1") });
    assert_eq!(*core_body, Core::Id { lit: String::from("body") });
}

#[test]
fn range_verify() {
    let from = to_pos!(ASTNode::Id { lit: String::from("a") });
    let to = to_pos!(ASTNode::Id { lit: String::from("b") });
    let range = to_pos!(ASTNode::Range { from, to, inclusive: false, step: None });

    let (from, to, step) = match desugar(&range) {
        Core::Range { from, to, step } => (from, to, step),
        other => panic!("Expected range but was {:?}", other)
    };

    assert_eq!(*from, Core::Id { lit: String::from("a") });
    assert_eq!(*to, Core::Id { lit: String::from("b") });
    assert_eq!(*step, Core::Int { int: String::from("1") });
}

#[test]
fn range_incl_verify() {
    let from = to_pos!(ASTNode::Id { lit: String::from("a") });
    let to = to_pos!(ASTNode::Id { lit: String::from("b") });
    let range = to_pos!(ASTNode::Range { from, to, inclusive: true, step: None });

    let (from, to, step) = match desugar(&range) {
        Core::Range { from, to, step } => (from, to, step),
        other => panic!("Expected range but was {:?}", other)
    };

    assert_eq!(*from, Core::Id { lit: String::from("a") });
    assert_eq!(*to, Core::Add {
        left:  Box::from(Core::Id { lit: String::from("b") }),
        right: Box::from(Core::Int { int: String::from("1") })
    });
    assert_eq!(*step, Core::Int { int: String::from("1") });
}

#[test]
fn range_step_verify() {
    let from = to_pos!(ASTNode::Id { lit: String::from("a") });
    let to = to_pos!(ASTNode::Id { lit: String::from("b") });
    let step = Some(to_pos!(ASTNode::Id { lit: String::from("c") }));
    let range = to_pos!(ASTNode::Range { from, to, inclusive: false, step });

    let (from, to, step) = match desugar(&range) {
        Core::Range { from, to, step } => (from, to, step),
        other => panic!("Expected range but was {:?}", other)
    };

    assert_eq!(*from, Core::Id { lit: String::from("a") });
    assert_eq!(*to, Core::Id { lit: String::from("b") });
    assert_eq!(*step, Core::Id { lit: String::from("c") });
}
