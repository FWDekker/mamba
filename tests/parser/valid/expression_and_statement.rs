use mamba::lexer::tokenize;
use mamba::parser::ast::ASTNode;
use mamba::parser::parse;
use mamba::parser::parse_direct;

#[test]
fn quest_or_verify() {
    let source = String::from("a ?or b");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let (_do, _default) = match ast_tree.node {
        ASTNode::Script { statements, .. } =>
            match &statements.first().expect("script empty.").node {
                ASTNode::QuestOr { _do, _default } => (_do.clone(), _default.clone()),
                _ => panic!("first element script was not list.")
            },
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(_do.node, ASTNode::Id { lit: String::from("a") });
    assert_eq!(_default.node, ASTNode::Id { lit: String::from("b") });
}

#[test]
fn range_verify() {
    let source = String::from("hello .. world");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let (from, to) = match ast_tree.node {
        ASTNode::Script { statements, .. } =>
            match &statements.first().expect("script empty.").node {
                ASTNode::Range { from, to } => (from.clone(), to.clone()),
                _ => panic!("first element script was not range.")
            },
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(from.node, ASTNode::Id { lit: String::from("hello") });
    assert_eq!(to.node, ASTNode::Id { lit: String::from("world") });
}

#[test]
fn range_incl_verify() {
    let source = String::from("foo ..= bar");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let (from, to) = match ast_tree.node {
        ASTNode::Script { statements, .. } =>
            match &statements.first().expect("script empty.").node {
                ASTNode::RangeIncl { from, to } => (from.clone(), to.clone()),
                _ => panic!("first element script was not range inclusive.")
            },
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(from.node, ASTNode::Id { lit: String::from("foo") });
    assert_eq!(to.node, ASTNode::Id { lit: String::from("bar") });
}

#[test]
fn reassign_verify() {
    let source = String::from("id <- new_value");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let (left, right) = match ast_tree.node {
        ASTNode::Script { statements, .. } =>
            match &statements.first().expect("script empty.").node {
                ASTNode::Reassign { left, right } => (left.clone(), right.clone()),
                _ => panic!("first element script was not reassign.")
            },
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(left.node, ASTNode::Id { lit: String::from("id") });
    assert_eq!(right.node, ASTNode::Id { lit: String::from("new_value") });
}

#[test]
fn print_verify() {
    let source = String::from("print some_value");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let expr = match ast_tree.node {
        ASTNode::Script { statements, .. } =>
            match &statements.first().expect("script empty.").node {
                ASTNode::Print { expr } => expr.clone(),
                _ => panic!("first element script was not reassign.")
            },
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(expr.node, ASTNode::Id { lit: String::from("some_value") });
}

#[test]
fn return_verify() {
    let source = String::from("return some_value");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let expr = match ast_tree.node {
        ASTNode::Script { statements, .. } =>
            match &statements.first().expect("script empty.").node {
                ASTNode::Return { expr } => expr.clone(),
                _ => panic!("first element script was not reassign.")
            },
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(expr.node, ASTNode::Id { lit: String::from("some_value") });
}

#[test]
fn retry_verify() {
    let source = String::from("retry");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let node_pos = match ast_tree.node {
        ASTNode::Script { statements, .. } => statements.first().expect("script empty.").clone(),
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(node_pos.node, ASTNode::Retry);
}

#[test]
fn underscore_verify() {
    let source = String::from("_");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let node_pos = match ast_tree.node {
        ASTNode::Script { statements, .. } => statements.first().expect("script empty.").clone(),
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(node_pos.node, ASTNode::Underscore);
}

#[test]
fn pass_verify() {
    let source = String::from("pass");
    let ast_tree = parse_direct(&tokenize(&source).unwrap()).unwrap();

    let node_pos = match ast_tree.node {
        ASTNode::Script { statements, .. } => statements.first().expect("script empty.").clone(),
        _ => panic!("ast_tree was not script.")
    };

    assert_eq!(node_pos.node, ASTNode::Pass);
}

#[test]
fn from_import_verify() {
    let source = String::from("from a import b");
    let ast_tree = parse(&tokenize(&source).unwrap()).unwrap();

    let imports = match ast_tree.node {
        ASTNode::File { imports, .. } => imports,
        _ => panic!("ast_tree was not file.")
    };

    assert_eq!(imports.len(), 1);
    let (id, _use, _as) = match &imports[0].node {
        ASTNode::FromImport { id, import } => match &import.node {
            ASTNode::Import { _use, _as } => (id, _use, _as),
            other => panic!("Expected import but was {:?}.", other)
        }
        other => panic!("Expected from import but was {:?}.", other)
    };

    assert_eq!(id.node, ASTNode::Id { lit: String::from("a") });
    assert_eq!(_use.len(), 1);
    assert_eq!(_use[0].node, ASTNode::Id { lit: String::from("b") });
    assert_eq!(_as.is_none(), true);
}

#[test]
fn import_verify() {
    let source = String::from("import c");
    let ast_tree = parse(&tokenize(&source).unwrap()).unwrap();

    let imports = match ast_tree.node {
        ASTNode::File { imports, .. } => imports,
        _ => panic!("ast_tree was not file.")
    };

    assert_eq!(imports.len(), 1);
    let (_use, _as) = match &imports[0].node {
        ASTNode::Import { _use, _as } => (_use, _as),
        other => panic!("Expected import but was {:?}.", other)
    };

    assert_eq!(_use.len(), 1);
    assert_eq!(_use[0].node, ASTNode::Id { lit: String::from("c") });
    assert_eq!(_as.is_none(), true);
}

#[test]
fn import_as_verify() {
    let source = String::from("import a, b as c, d");
    let ast_tree = parse(&tokenize(&source).unwrap()).unwrap();

    let imports = match ast_tree.node {
        ASTNode::File { imports, .. } => imports,
        _ => panic!("ast_tree was not file.")
    };

    assert_eq!(imports.len(), 1);
    let (_use, _as) = match &imports[0].node {
        ASTNode::Import { _use, _as: Some(thing) } => (_use, thing),
        other => panic!("Expected import but was {:?}.", other)
    };

    assert_eq!(_use.len(), 2);
    assert_eq!(_use[0].node, ASTNode::Id { lit: String::from("a") });
    assert_eq!(_use[1].node, ASTNode::Id { lit: String::from("b") });
    assert_eq!(_as.len(), 2);
    assert_eq!(_as[0].node, ASTNode::Id { lit: String::from("c") });
    assert_eq!(_as[1].node, ASTNode::Id { lit: String::from("d") });
}