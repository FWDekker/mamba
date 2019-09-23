use crate::core::construct::Core;
use crate::desugar::context::Imports;
use crate::desugar::context::State;
use crate::desugar::desugar_result::DesugarResult;
use crate::desugar::node::desugar_node;
use crate::parser::ast::AST;

pub fn desugar_vec(node_vec: &[AST], imp: &mut Imports, state: &State) -> DesugarResult<Vec<Core>> {
    let mut result = vec![];
    for ast in node_vec {
        result.push(desugar_node(ast, imp, state)?)
    }

    Ok(result)
}
