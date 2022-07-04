use crate::lexer::TokenKind;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Bool(bool),
    Nil,
    Var(Identifier, Box<Expression>),
    Binary(Box<Expression>, TokenKind, Box<Expression>),
    Grouping(Box<Expression>),
    Unary(TokenKind, Box<Expression>),
}
