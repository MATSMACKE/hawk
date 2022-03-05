use crate::token::{TokenType, Object};

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Box<Expression>),
    Declaration{},
    EOF,
    Expression(Box<Expression>)
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Object),
    Unary{
        operand: Box<Expression>, 
        operator: TokenType
    },
    Binary{
        operand1: Box<Expression>, 
        operand2: Box<Expression>, 
        operator: TokenType
    },
    Parenthesized(Box<Expression>)
}