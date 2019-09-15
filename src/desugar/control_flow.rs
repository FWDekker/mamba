use crate::core::construct::Core;
use crate::desugar::context::Imports;
use crate::desugar::context::State;
use crate::desugar::desugar_result::DesugarResult;
use crate::desugar::desugar_result::UnimplementedErr;
use crate::desugar::node::desugar_node;
use crate::parser::ast::Node;
use crate::parser::ast::AST;

pub fn desugar_control_flow(node_pos: &AST, imp: &mut Imports, state: &State) -> DesugarResult {
    Ok(match &node_pos.node {
        Node::IfElse { cond, then, _else } => match _else {
            Some(_else) => Core::IfElse {
                cond:  Box::from(desugar_node(cond, imp, state)?),
                then:  Box::from(desugar_node(then, imp, state)?),
                _else: Box::from(desugar_node(_else, imp, state)?)
            },
            None => Core::If {
                cond: Box::from(desugar_node(cond, imp, state)?),
                then: Box::from(desugar_node(then, imp, state)?)
            }
        },
        Node::Match { cond, cases } => {
            let expr = Box::from(desugar_node(cond, imp, state)?);
            let mut core_cases = vec![];
            let mut core_defaults = vec![];

            for case in cases {
                match &case.node {
                    Node::Case { cond, body } => match &cond.node {
                        Node::IdType { id, .. } => match id.node {
                            Node::Underscore =>
                                core_defaults.push(desugar_node(body.as_ref(), imp, state)?),
                            _ => core_cases.push(Core::KeyValue {
                                key:   Box::from(desugar_node(cond.as_ref(), imp, state)?),
                                value: Box::from(desugar_node(body.as_ref(), imp, state)?)
                            })
                        },
                        _ =>
                            return Err(UnimplementedErr::new(
                                node_pos,
                                "match case expression as condition (pattern matching)"
                            )),
                    },
                    other => panic!("Expected case but was {:?}", other)
                }
            }

            if core_defaults.len() > 1 {
                panic!("Can't have more than one default.")
            } else if core_defaults.len() == 1 {
                let default = Box::from(Core::AnonFun {
                    args: vec![],
                    body: Box::from(core_defaults[0].clone())
                });

                imp.add_from_import("collections", "defaultdict");
                Core::DefaultDictionary { expr, cases: core_cases, default }
            } else {
                Core::Dictionary { expr, cases: core_cases }
            }
        }
        Node::While { cond, body } => Core::While {
            cond: Box::from(desugar_node(cond, imp, state)?),
            body: Box::from(desugar_node(body, imp, state)?)
        },
        Node::For { expr, body } => Core::For {
            expr: Box::from(desugar_node(expr, imp, state)?),
            body: Box::from(desugar_node(body, imp, state)?)
        },

        Node::Break => Core::Break,
        Node::Continue => Core::Continue,
        other => panic!("Expected control flow but was: {:?}.", other)
    })
}
