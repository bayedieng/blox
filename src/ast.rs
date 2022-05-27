use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),

    Binary {
        lhs: Box<Expression>,
        operator: Token,
        rhs: Box<Expression>,
    },

    Grouping(Box<Expression>),

    Unary {
        operator: Token,
        expression: Box<Expression>,
    },
}
