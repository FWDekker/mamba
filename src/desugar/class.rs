use std::ops::Deref;

use crate::core::construct::Core;
use crate::desugar::common::desugar_vec;
use crate::desugar::desugar_result::DesugarResult;
use crate::desugar::node::desugar_node;
use crate::desugar::state::Imports;
use crate::desugar::state::State;
use crate::desugar::ty::desugar_type;
use crate::parser::ast::Node;
use crate::parser::ast::AST;
use crate::type_checker::context::{function, function_arg};

/// Desugar a class.
///
/// If a class has inline arguments (arguments next to class), then we create a
/// constructor and assume that there is no constructor in the body of a class.
/// This property should be ensured by the type checker.
///
/// We add arguments and calls to super for parents.
pub fn desugar_class(ast: &AST, imp: &mut Imports, state: &State) -> DesugarResult {
    Ok(match &ast.node {
        Node::TypeDef { _type, body: Some(body), isa } => match (&_type.node, &body.node) {
            (Node::Type { id, .. }, Node::Block { statements }) => Core::ClassDef {
                name:        Box::from(desugar_node(id, imp, state)?),
                parents:     if let Some(isa) = isa {
                    vec![desugar_node(isa, imp, state)?]
                } else {
                    vec![]
                },
                definitions: desugar_vec(statements, imp, &state.in_interface(true))?
            },
            other => panic!("desugar didn't recognize while making type definition: {:?}.", other)
        },
        Node::TypeDef { _type, body: None, isa } => match &_type.node {
            Node::Type { id, .. } => Core::ClassDef {
                name:        Box::from(desugar_node(id, imp, state)?),
                parents:     if let Some(isa) = isa {
                    vec![desugar_node(isa, imp, state)?]
                } else {
                    vec![]
                },
                definitions: vec![]
            },
            other => panic!("desugar didn't recognize while making type definition: {:?}.", other)
        },

        Node::Class { _type, body, args, parents } => {
            let statements = if let Some(body) = body {
                match &body.deref().node {
                    Node::Block { statements } => statements.clone(),
                    _ => vec![]
                }
            } else {
                vec![]
            };

            match &_type.node {
                Node::Type { id, .. } => {
                    let (parent_names, parent_args, super_calls) =
                        extract_parents(parents, imp, state)?;
                    let core_definitions: Vec<Core> = desugar_vec(&statements, imp, state)?;
                    let inline_args = desugar_vec(args, imp, state)?;

                    let final_definitions = if parent_names.is_empty() && inline_args.is_empty() {
                        desugar_vec(&statements, imp, state)?
                    } else {
                        let (found_constructor, augmented_definitions) =
                            add_parent_to_constructor(&core_definitions, &super_calls)?;

                        if found_constructor && !args.is_empty() {
                            panic!("Cannot have explicit constructor and inline arguments.")
                        } else if found_constructor {
                            augmented_definitions
                        } else {
                            constructor_from_inline(
                                &inline_args,
                                &parent_args,
                                &super_calls,
                                &augmented_definitions
                            )?
                        }
                    };

                    let mut final_definitions = if final_definitions.is_empty() {
                        vec![Core::FunDef {
                            private: false,
                            id:      Box::new(Core::Id {
                                lit: String::from(function::python::INIT)
                            }),
                            args:    vec![Core::FunArg {
                                vararg:  false,
                                id:      Box::new(Core::Id {
                                    lit: String::from(function_arg::python::SELF)
                                }),
                                default: Box::new(Core::Empty)
                            }],
                            ret_ty:  None,
                            body:    Box::new(Core::Pass)
                        }]
                    } else {
                        final_definitions
                    };

                    let (mut stmts, mut non_variables): (Vec<_>, Vec<_>) =
                        final_definitions.into_iter().partition(|stmt| match stmt {
                            Core::VarDef { .. } => true,
                            _ => false
                        });
                    stmts.append(&mut non_variables);
                    final_definitions = stmts.clone();

                    Core::ClassDef {
                        name:        Box::from(desugar_node(id, imp, state)?),
                        parents:     parent_names,
                        definitions: final_definitions
                    }
                }
                other => panic!("Didn't recognize while making class: {:?}.", other)
            }
        }
        other => panic!("Expected class or type definition but was {:?}", other)
    })
}

// TODO simplify application logic
fn constructor_from_inline(
    inline_args: &[Core],
    parent_args: &[Core],
    super_calls: &[Core],
    definitions: &[Core]
) -> DesugarResult<Vec<Core>> {
    let mut final_definitions = vec![];
    let mut args = vec![Core::Id { lit: String::from("self") }];
    let mut statements = Vec::from(super_calls);

    for inline_arg in inline_args {
        match inline_arg {
            Core::FunArg { id, .. } => {
                args.push(inline_arg.clone());
                if !parent_args.contains(&match id.deref() {
                    Core::IdType { lit, .. } => Core::Id { lit: lit.clone() },
                    other => other.clone()
                }) {
                    final_definitions
                        .push(Core::Assign { left: id.clone(), right: Box::from(Core::None) })
                }
            }

            Core::VarDef { id, right, .. } => {
                args.push(Core::FunArg {
                    vararg:  false,
                    id:      id.clone(),
                    default: match &right.deref() {
                        Core::None => Box::from(Core::Empty),
                        _ => right.clone()
                    }
                });

                let id = match &id.deref() {
                    Core::IdType { lit, .. } => Box::from(Core::Id { lit: lit.clone() }),
                    _ => id.clone()
                };

                if !parent_args.contains(&match id.deref() {
                    Core::IdType { lit, .. } => Core::Id { lit: lit.clone() },
                    other => other.clone()
                }) {
                    final_definitions
                        .push(Core::Assign { left: id.clone(), right: Box::from(Core::None) });
                    statements.push(Core::Assign {
                        left:  Box::from(Core::PropertyCall {
                            object:   Box::new(Core::Id { lit: String::from("self") }),
                            property: id.clone()
                        }),
                        right: Box::from(match id.deref() {
                            Core::IdType { lit, .. } => Core::Id { lit: lit.clone() },
                            id => id.clone()
                        })
                    });
                }
            }
            _ => panic!("Inline arg was not function argument: {:?}", inline_arg)
        }
    }

    let id = Box::from(Core::Id { lit: String::from("init") });
    let body = Box::from(Core::Block { statements });
    let core_init = Core::FunDef { private: false, id, args, ret_ty: None, body };

    final_definitions.push(core_init);
    final_definitions.append(&mut Vec::from(definitions));
    Ok(final_definitions)
}

fn add_parent_to_constructor(
    core_definitions: &[Core],
    super_calls: &[Core]
) -> DesugarResult<(bool, Vec<Core>)> {
    let mut final_definitions = vec![];
    let mut found_constructor = false;

    for definition in core_definitions {
        final_definitions.push(
            if let Core::FunDef { private, id, args, body: old_body, .. } = definition {
                if let Core::Id { lit, .. } = id.clone().deref() {
                    if lit == "init" {
                        if found_constructor {
                            panic!("Cannot have more than one constructor.")
                        }
                        found_constructor = true;
                        let body = match (super_calls.is_empty(), *old_body.clone()) {
                            (true, _) => old_body.clone(),
                            (false, Core::Block { statements: old_statements }) => {
                                let mut statements = Vec::from(super_calls);
                                statements.append(&mut old_statements.clone());
                                Box::from(Core::Block { statements })
                            }
                            (false, core) => {
                                let mut statements = Vec::from(super_calls);
                                statements.push(core);
                                Box::from(Core::Block { statements })
                            }
                        };

                        Core::FunDef {
                            private: *private,
                            id: id.clone(),
                            args: args.clone(),
                            ret_ty: None,
                            body
                        }
                    } else {
                        definition.clone()
                    }
                } else {
                    definition.clone()
                }
            } else {
                definition.clone()
            }
        );
    }

    Ok((found_constructor, final_definitions))
}

fn extract_parents(
    parents: &[AST],
    ctx: &mut Imports,
    state: &State
) -> DesugarResult<(Vec<Core>, Vec<Core>, Vec<Core>)> {
    let mut parent_names: Vec<Core> = vec![];
    let mut parent_args: Vec<Core> = vec![];
    let mut super_calls: Vec<Core> = vec![];

    for parent in parents {
        match &parent.node {
            Node::Parent { ref id, args: old_args, .. } => {
                parent_names.push(desugar_type(id, ctx, state)?);

                let mut args = vec![];
                args.append(&mut desugar_vec(old_args, ctx, state)?);
                parent_args.append(&mut desugar_vec(old_args, ctx, state)?);

                super_calls.push(Core::PropertyCall {
                    object:   Box::from(Core::FunctionCall {
                        function: Box::from(Core::Id { lit: String::from("super") }),
                        args:     vec![]
                    }),
                    property: Box::from(Core::FunctionCall {
                        function: Box::from(Core::Id { lit: String::from("__init__") }),
                        args
                    })
                });
            }
            other => panic!("Expected parent but got {:?}", other)
        }
    }

    Ok((parent_names, parent_args, super_calls))
}
