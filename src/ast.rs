use crate::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),

    Binary {
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
    },

    Grouping(Box<Expression>),

    Unary {
        operator: TokenKind,
        expression: Box<Expression>,
    },
}
