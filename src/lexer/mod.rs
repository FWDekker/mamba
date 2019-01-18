use crate::lexer::token::Token;
use crate::lexer::token::TokenPos;
use std::iter::Peekable;
use std::str::Chars;

pub mod token;

#[macro_use]
macro_rules! next_and { ($it:expr, $pos:expr, $stmt:stmt) => {{ $it.next(); *$pos += 1; $stmt }} }

pub fn tokenize(input: String) -> Result<Vec<TokenPos>, String> {
    let mut it = input.chars().peekable();
    let mut tokens = Vec::new();

    let mut current_indent = 0;
    let mut this_line_indent = 0;
    let mut consecutive_spaces = 0;

    let mut line = 1;
    let mut pos = 1;

    macro_rules! next_pos_and_tp {
    ($amount:expr, $tok:path) => {{
        for _ in current_indent..this_line_indent {
            tokens.push(TokenPos { line, pos, token: Token::Indent });
        }
        for _ in this_line_indent..current_indent {
            tokens.push(TokenPos { line, pos, token: Token::Dedent });
        }
        tokens.push(TokenPos { line, pos, token: Token::NL });

        it.next();
        tokens.push(TokenPos { line, pos, token: $tok });
        pos += $amount;

        current_indent = this_line_indent;
    }};
    ($fun:path) => {{
        for _ in current_indent..this_line_indent {
            tokens.push(TokenPos { line, pos, token: Token::Indent });
        }
        for _ in this_line_indent..current_indent {
            tokens.push(TokenPos { line, pos, token: Token::Dedent });
        }

        tokens.push(TokenPos { line, pos, token: $fun(&mut it, &mut pos) });

        current_indent = this_line_indent;
    }}
    };

    macro_rules! next_line_and_tp { () => {{
        tokens.push(TokenPos { line, pos, token: Token::NL });

        it.next();
        line += 1;
        pos = 1;

        current_indent = this_line_indent;
        this_line_indent = 0;
    }}};

    macro_rules! increase_indent { () => {{
        line += 1;
        pos += 4;
        this_line_indent += 1;
    }}};

    while let Some(&c) = it.peek() {
        match c {
            '.' => next_pos_and_tp!(1, Token::Point),
            ':' => {
                it.next();
                match it.peek() {
                    Some(':') => next_pos_and_tp!(2, Token::DDoublePoint),
                    _ => next_pos_and_tp!(1, Token::DoublePoint),
                }
            }
            ',' => next_pos_and_tp!(1, Token::Comma),
            '(' => next_pos_and_tp!(1, Token::LPar),
            ')' => next_pos_and_tp!(1, Token::RPar),
            '[' => next_pos_and_tp!(1, Token::LBrack),
            ']' => next_pos_and_tp!(1, Token::RBrack),
            '{' => next_pos_and_tp!(1, Token::LCurl),
            '}' => next_pos_and_tp!(1, Token::RCurl),
            '|' => next_pos_and_tp!(1, Token::Ver),
            '\n' => next_line_and_tp!(),
            '\r' => {
                it.next();
                match it.peek() {
                    Some('\n') => next_line_and_tp!(),
                    Some(other) =>
                        return Err(format!("Expected newline after carriage return. Was {}", other)),
                    None => return Err("File ended with carriage return".to_string())
                }
            }
            '\t' => increase_indent!(),
            '<' | '>' | '+' | '-' | '*' | '/' | '^' => next_pos_and_tp!(get_operator),
            '0'...'9' => next_pos_and_tp!(get_number),
            '"' => next_pos_and_tp!(get_string),
            'a'...'z' | 'A'...'Z' => next_pos_and_tp!(get_id_or_op),
            '#' => ignore_comment(&mut it),
            ' ' => {
                pos += 1;
                consecutive_spaces += 1;
                if consecutive_spaces == 4 {
                    consecutive_spaces = 0;
                    increase_indent!();
                    pos -= 4;
                }
                it.next();
                continue;
            }
            c => return Err(format!("Unrecognized character whilst tokenizing: '{}'.", c)),
        }

        consecutive_spaces = 0;
    }

    return Ok(tokens);
}

fn ignore_comment(it: &mut Peekable<Chars>) {
    while let Some(c) = it.peek() {
        match c {
            '\n' => break,
            _ => {
                it.next();
                continue;
            }
        }
    }
}

fn get_operator(it: &mut Peekable<Chars>, pos: &mut i32) -> Token {
    *pos += 1;
    return match it.next() {
        Some('<') => match it.peek() {
            Some('=') => next_and!(it, pos, Token::Leq),
            Some('-') => next_and!(it, pos, Token::Assign),
            _ => Token::Le
        }
        Some('>') => match it.peek() {
            Some('=') => next_and!(it, pos, Token::Geq),
            _ => Token::Ge
        }
        Some('+') => Token::Add,
        Some('-') => Token::Sub,
        Some('/') => Token::Div,
        Some('*') => Token::Mul,
        Some('^') => Token::Pow,
        _ => panic!("get operator received a character it shouldn't have.")
    };
}

fn get_number(it: &mut Peekable<Chars>, pos: &mut i32) -> Token {
    let mut num = String::new();
    let mut exp = String::new();
    let mut e_found = false;
    let mut comma = false;

    match it.next() {
        Some(digit @ '0'...'9') => num.push(digit),
        _ => panic!("get number received a character it shouldn't have.")
    }
    *pos += 1;

    while let Some(&c) = it.peek() {
        match c {
            '0'...'9' if !e_found => next_and!(it, pos, num.push(c)),
            '0'...'9' if e_found => next_and!(it, pos, exp.push(c)),
            'e' | 'E' if e_found => break,
            'e' | 'E' => next_and!(it, pos, e_found = true),
            '.' if comma || e_found => break,
            '.' => next_and!(it, pos, { num.push(c); comma = true; }),
            _ => break
        }
    }

    return match (e_found, comma) {
        (true, _) => Token::ENum(num, exp),
        (false, true) => Token::Real(num),
        (false, false) => Token::Int(num)
    };
}

fn get_string(it: &mut Peekable<Chars>, pos: &mut i32) -> Token {
    it.next();
    let mut result = String::new();

    while let Some(&c) = it.peek() {
        match c {
            '"' => next_and!(it, pos, break),
            _ => next_and!(it, pos, result.push(c))
        }
    }

    return Token::Str(result);
}

fn get_id_or_op(it: &mut Peekable<Chars>, pos: &mut i32) -> Token {
    let mut result = String::new();
    *pos += 1;

    while let Some(&c) = it.peek() {
        match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => next_and!(it, pos, result.push(c)),
            _ => break
        }
    }

    return match result.as_ref() {
        "util" => Token::Util,
        "type" => Token::Type,
        "as" => Token::As,
        "isa" => Token::IsA,
        "constructor" => Token::Constructor,

        "use" => Token::Use,
        "useall" => Token::UseAll,
        "class" => Token::Class,
        "forward" => Token::Forward,
        "self" => Token::_Self,

        "fun" => Token::Fun,
        "def" => Token::Def,
        "mutable" => Token::Mut,
        "and" => Token::And,
        "or" => Token::Or,
        "not" => Token::Not,
        "is" => Token::Is,
        "isnot" => Token::IsN,
        "equals" => Token::Eq,
        "notequals" => Token::Neq,
        "mod" => Token::Mod,
        "sqrt" => Token::Sqrt,
        "while" => Token::While,
        "for" => Token::For,
        "where" => Token::Where,

        "in" => Token::In,
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "unless" => Token::Unless,
        "when" => Token::When,
        "do" => Token::Do,
        "continue" => Token::Continue,
        "break" => Token::Break,
        "return" => Token::Ret,

        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "print" => Token::Print,
        _ => Token::Id(result)
    };
}
