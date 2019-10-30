use crate::lexer::token::Token;
use crate::parser::ast::Node;
use crate::parser::ast::AST;
use crate::parser::expression::is_start_expression;
use crate::parser::iterator::LexIterator;
use crate::parser::operation::parse_operation;
use crate::parser::parse_result::expected_one_of;
use crate::parser::parse_result::ParseResult;

pub fn parse_collection(it: &mut LexIterator) -> ParseResult {
    it.peek_or_err(
        &|it, lex| match lex.token {
            Token::LRBrack => parse_tuple(it),
            Token::LSBrack => parse_list(it),
            Token::LCBrack => parse_set(it),
            _ => Err(expected_one_of(
                &[Token::LRBrack, Token::LSBrack, Token::LCBrack],
                lex,
                "collection"
            ))
        },
        &[Token::LRBrack, Token::LSBrack, Token::LCBrack],
        "collection"
    )
}

pub fn parse_tuple(it: &mut LexIterator) -> ParseResult {
    let start = it.start_pos("tuple")?;
    it.eat(&Token::LRBrack, "tuple")?;
    let elements = it.parse_vec(&parse_expressions, "tuple", &start)?;
    let end = it.eat(&Token::RRBrack, "tuple")?;

    Ok(Box::from(if elements.len() == 1 {
        elements[0].clone()
    } else {
        AST::new(&start.union(&end), Node::Tuple { elements })
    }))
}

fn parse_set(it: &mut LexIterator) -> ParseResult {
    let start = it.start_pos("set")?;
    it.eat(&Token::LCBrack, "set")?;

    if let Some(end) = it.eat_if(&Token::RCBrack) {
        let node = Node::Set { elements: vec![] };
        return Ok(Box::from(AST::new(&start.union(&end), node)));
    }

    let item = it.parse(&parse_operation, "set", &start)?;
    if it.eat_if(&Token::Ver).is_some() {
        let conditions = it.parse_vec(&parse_expressions, "set", &start)?;
        let end = it.eat(&Token::RCBrack, "set")?;
        let node = Node::SetBuilder { item, conditions };
        return Ok(Box::from(AST::new(&start.union(&end), node)));
    }

    let mut elements = vec![*item];
    elements.append(&mut it.parse_vec_if(&Token::Comma, &parse_expressions, "set", &start)?);

    let end = it.eat(&Token::RCBrack, "set")?;
    let node = Node::Set { elements };
    Ok(Box::from(AST::new(&start.union(&end), node)))
}

fn parse_list(it: &mut LexIterator) -> ParseResult {
    let start = it.start_pos("list")?;
    it.eat(&Token::LSBrack, "list")?;

    if let Some(end) = it.eat_if(&Token::RSBrack) {
        let node = Node::List { elements: vec![] };
        return Ok(Box::from(AST::new(&start.union(&end), node)));
    }

    let item = it.parse(&parse_operation, "list", &start)?;
    if it.eat_if(&Token::Ver).is_some() {
        let conditions = it.parse_vec(&parse_expressions, "list", &start)?;
        let end = it.eat(&Token::RSBrack, "list")?;
        let node = Node::ListBuilder { item, conditions };
        return Ok(Box::from(AST::new(&start.union(&end), node)));
    }

    let mut elements = vec![*item];
    elements.append(&mut it.parse_vec_if(&Token::Comma, &parse_expressions, "list", &start)?);

    let end = it.eat(&Token::RSBrack, "list")?;
    let node = Node::List { elements };
    Ok(Box::from(AST::new(&start.union(&end), node)))
}

pub fn parse_expressions(it: &mut LexIterator) -> ParseResult<Vec<AST>> {
    let start = it.start_pos("expression")?;
    let mut expressions = vec![];
    it.peek_while_fn(&is_start_expression, &mut |it, _| {
        expressions.push(*it.parse(&parse_operation, "expressions", &start)?);
        it.eat_if(&Token::Comma);
        Ok(())
    })?;
    Ok(expressions)
}
