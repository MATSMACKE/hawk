use crate::token::{TokenType};
use crate::object::Object;

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Box<Expression>),
    Definition{name: String, value: Box<Expression>},
    Block(Vec<Statement>),
    EOF,
    Expression(Box<Expression>),
    While{condition: Box<Expression>, block: Box<Statement>},
    Loop(Box<Statement>),
    Break,
    If{condition: Box<Expression>, block: Box<Statement>},
    IfElse{condition: Box<Expression>, if_block: Box<Statement>, else_block: Box<Statement>},
    Function{identifier: String, params: Vec<String>, block: Box<Statement>},
    Return(Box<Expression>),
    Import(Box<Expression>)
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
    Parenthesized(Box<Expression>),
    FunctionCall{identifier: String, args: Vec<Box<Expression>>},
    Array(Vec<Box<Expression>>)
}