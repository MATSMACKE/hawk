use crate::token::{TokenType};

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Parenthesized(Box<Expression>)
}

#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    Boolean(bool),
    Int(isize),
    Float(isize),
    String(String)
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub operand1: Box<Expression>, 
    pub operand2: Box<Expression>, 
    pub operator: TokenType
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operand: Box<Expression>, 
    pub operator: TokenType
}