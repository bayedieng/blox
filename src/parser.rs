// Parse Tokens into AST

use crate::lexer::{Lexer, TokenKind};
use crate::ast::*;

#[derive(Debug)]
pub enum ParseError {
    SyntaxError
}

pub type ParseResult = Result<Expression, ParseError>;


pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(src: &str) -> Parser {
        Parser {
            lexer: Lexer::new(src)
        }
    }

    pub fn parse_primary(&mut self) -> ParseResult {
        let token = self.lexer.next_token();
        match token.kind {
            TokenKind::Number(val) => Ok(Expression::Number(val)),
            TokenKind::String(val) => Ok(Expression::String(val)),
            TokenKind::True => Ok(Expression::Bool(true)),
            TokenKind::False => Ok(Expression::Bool(false)),
            _ => Err(ParseError::SyntaxError)
        }
    }
}