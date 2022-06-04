use crate::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Bool(bool),
    Nil,

    Binary(Box<Expression>, TokenKind, Box<Expression>),

    Grouping(Box<Expression>),

    Unary(TokenKind, Box<Expression>),
}
