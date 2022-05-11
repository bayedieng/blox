use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Binary {
        lhs: Box<Expression>,
        operator: Token,
        rhs: Box<Expression>
    },
    
    Bool(bool),
    Number(f64),
    Nil(f64),
    String(String),

    Unary(Token, Box<Expression>),
    Grouping(Box<Expression>),
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExprStmt(Expression),
    PrintStmt(Expression)
}