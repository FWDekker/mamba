use mamba::check::check_all;
use mamba::generate::gen;
use mamba::parse::parse;

use crate::common::*;

#[test]
fn core_function_definitions() {
    let source = resource_content(true, &["function"], "definition.mamba");
    to_py!(source);
}

#[test]
fn core_function_calling() {
    let source = resource_content(true, &["function"], "calls.mamba");
    to_py!(source);
}
