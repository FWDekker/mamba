use std::path::PathBuf;

use crate::lexer::token::Lex;
use crate::parser::iterator::LexIterator;
use crate::parser::parse_result::{ParseResult, ParseResults};

pub mod ast;

mod iterator;

pub mod parse_result;

mod _type;
mod block;
mod call;
mod class;
mod collection;
mod control_flow_expr;
mod control_flow_stmt;
mod definition;
mod expr_or_stmt;
mod expression;
mod file;
mod operation;
mod statement;

pub type ParseInput = (Vec<Lex>, Option<String>, Option<PathBuf>);

/// Parse input, which is a slice of [TokenPos](crate::lexer::token::TokenPos).
///
/// Should never panic.
///
/// # Examples
///
/// ```
/// # use mamba::lexer::token::Token;
/// # use mamba::lexer::token::Lex;
/// # use mamba::parser::parse;
/// # use mamba::common::position::CaretPos;
/// let def = Lex::new(&CaretPos::new(1, 1), Token::Def);
/// let id = Lex::new(&CaretPos::new(1, 4), Token::Id(String::from("b")));
/// let assign = Lex::new(&CaretPos::new(1, 6), Token::Assign);
/// let number = Lex::new(&CaretPos::new(1, 9), Token::Int(String::from("9")));
///
/// let result = parse(&[def, id, assign, number]);
/// assert_eq!(result.is_ok(), true);
/// ```
///
/// # Failures
///
/// If we receive an illegal sequence of tokens it fails.
///
/// ```
/// # use mamba::lexer::token::Token;
/// # use mamba::lexer::token::Lex;
/// # use mamba::parser::parse;
/// # use mamba::common::position::CaretPos;
/// let def = Lex::new(&CaretPos::new(0, 0), Token::Def);
/// let id = Lex::new(&CaretPos::new(0, 0), Token::Id(String::from("b")));
/// let number = Lex::new(&CaretPos::new(0, 0), Token::Int(String::from("9")));
///
/// let result = parse(&[def, id, number]);
/// assert_eq!(result.is_err(), true);
/// ```
pub fn parse(input: &[Lex]) -> ParseResult {
    file::parse_file(&mut LexIterator::new(input.iter().peekable()))
}

pub fn parse_all(inputs: &[ParseInput]) -> ParseResults {
    let inputs: Vec<_> = inputs
        .iter()
        .map(|(node_pos, source, path)| (parse(node_pos), source, path))
        .map(|(result, source, path)| {
            (result.map_err(|err| err.into_with_source(source, path)), source.clone(), path.clone())
        })
        .collect();

    let (oks, errs): (Vec<_>, Vec<_>) = inputs.iter().partition(|(res, ..)| res.is_ok());
    if errs.is_empty() {
        Ok(oks
            .iter()
            .map(|(res, src, path)| (*res.as_ref().unwrap().clone(), src.clone(), path.clone()))
            .collect())
    } else {
        Err(errs.iter().map(|(res, ..)| res.as_ref().unwrap_err().clone()).collect())
    }
}

/// Parse input as a script.
pub fn parse_direct(input: &[Lex]) -> ParseResult {
    file::parse_script(&mut LexIterator::new(input.iter().peekable()))
}
