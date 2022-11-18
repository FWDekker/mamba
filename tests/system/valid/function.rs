use crate::system::{OutTestRet, test_directory};

#[test]
fn call_ast_verify() -> OutTestRet {
    test_directory(true, &["function"], &["function", "target"], "calls")
}

#[test]
fn definition_ast_verify() -> OutTestRet {
    test_directory(true, &["function"], &["function", "target"], "definition")
}

#[test]
fn function_with_defaults_ast_verify() -> OutTestRet {
    test_directory(true, &["function"], &["function", "target"], "function_with_defaults")
}

#[test]
fn return_last_expression() -> OutTestRet {
    test_directory(true, &["function"], &["function", "target"], "return_last_expression")
}
