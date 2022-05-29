use crate::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),

    Binary {
        lhs: Box<Expression>,
        operator: TokenKind,
        rhs: Box<Expression>,
    },

    Grouping(Box<Expression>),

    Unary {
        operator: TokenKind,
        expression: Box<Expression>,
    },
}
