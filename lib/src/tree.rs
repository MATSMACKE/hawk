use std::fmt::{Display, Formatter, Result, Error};
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

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Self::Block(x) => write!(f, "Statement::Block(vec![{}])", Statements(x.clone())),
            Self::Break => write!(f, "Statement::Break"),
            Self::EOF => write!(f, "Statement::EOF"),
            Self::Definition{name, value} => write!(f, "Statement::Definition{{name: \"{}\".to_string(), value: Box::new({})}}", name, value),
            Self::Function{identifier, params, block} => write!(f, "Statement::Function{{identifier: \"{}\".to_string(), params: vec!{:?}.iter().map(|x| x.to_string()).collect(), block: Box::new({})}}", identifier, params, block),
            Self::Import(x) => write!(f, "Statement::Import(Box::new({}))", x),
            Self::If{condition, block} => write!(f, "Statement::If{{condition: Box::new({}), block: Box::new({})}}", condition, block),
            Self::IfElse{condition, if_block, else_block} => write!(f, "Statement::IfElse{{condition: Box::new({}), if_block: Box::new({}), else_block: Box::new({})}}", condition, if_block, else_block),
            Self::Return(x) => write!(f, "Statement::Return(Box::new({}))", x),
            Self::Print(x) => write!(f, "Statement::Print(Box::new({}))", x),
            Self::While{condition, block} => write!(f, "Statement::While{{condition: Box::new({}), block: Box::new({})}}", condition, block),
            Self::Loop(x) => write!(f, "Statement::Loop(Box::new({}))", x),
            Self::Expression(x) => write!(f, "Statement::Expression(Box::new({}))", x)
        }.unwrap();
        Ok(())
    }
}

impl Display for Statements {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut problem = false;
        for statement in self.0.iter() {
            if let Ok(_) = writeln!(f, "{},", statement) {
                ()
            } else {
                problem = true
            }
        }
        if problem {Err(Error)} else {Ok(())}
    }
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

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Self::Literal(x) => write!(f, "Expression::Literal({})", x),
            Self::Parenthesized(x) => write!(f, "Expression::Parenthesized({})", x),
            Self::Array(x) => write!(f, "Expression::Array({})", Expressions(x.clone())),
            Self::Unary{operand, operator} => write!(f, "Expression::Unary{{operand: Box::new({}), operator: {}}}", operand, operator),
            Self::Binary{operand1, operand2, operator} => write!(f, "Expression::Binary{{operand1: Box::new({}), operand2: Box::new({}), operator: {}}}", operand1, operand2, operator),
            Self::FunctionCall{identifier, args} => write!(f, "Expression::FunctionCall{{identifier: \"{}\".to_string(), args: vec![{}]}}", identifier, Expressions(args.clone()))
        }.unwrap();
        Ok(())
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut problem = false;
        for expression in self.0.iter() {
            if let Ok(_) = writeln!(f, "Box::new({})", expression) {
                ()
            } else {
                problem = true
            }
        }
        if problem {Err(Error)} else {Ok(())}
    }
}

pub struct Statements(pub Vec<Statement>);

pub struct Expressions(pub Vec<Box<Expression>>);