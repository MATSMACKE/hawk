use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter, Result};

use crate::object::Object;
use crate::token::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Print statement
    Print(Box<Expression>),
    /// Variable definition
    Definition{name: String, value: Box<Expression>},
    ArrayAssign{name: String, idx: Box<Expression>, value: Box<Expression>},
    /// Block of code (encased in `{}`)
    Block(Vec<Statement>),
    /// Indicates code has completed
    EOF,
    /// Expression statement (e.g. function call)
    Expression(Box<Expression>),
    /// While loop
    While{condition: Box<Expression>, block: Box<Statement>},
    /// Loop
    Loop(Box<Statement>),
    /// Break loop
    Break,
    /// If statement (no else)
    If{condition: Box<Expression>, block: Box<Statement>},
    /// If statement with `else`
    IfElse{condition: Box<Expression>, if_block: Box<Statement>, else_block: Box<Statement>},
    /// A statement that adds the parsed function to the top scope
    Function{identifier: String, params: Vec<String>, block: Box<Statement>},
    Finder{identifier: String, equations: Vec<(Expression, Expression)>},
    /// Returns function
    Return(Box<Expression>),
    /// Runs code from another file, importing functions and global variables
    Import(Box<Expression>),
    /// Process block to process and analyze data
    Process{readfile: Box<Expression>, writefile: Box<Expression>, block: Box<Statement>},
    /// Indicates new line
    Line,
}

// Implemented to print statements to Rust code for standard library compilation in `build.rs`
impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Self::Block(x) => writeln!(f, "Statement::Block(vec![{}])", Statements(x.clone())),
            Self::Break => writeln!(f, "Statement::Break"),
            Self::EOF => writeln!(f, "Statement::EOF"),
            Self::Definition{name, value} => writeln!(f, "Statement::Definition{{name: \"{}\".to_owned(), value: Box::new({})}}", name, value),
            Self::ArrayAssign{name, idx, value} => writeln!(f, "Statement::ArrayAssign{{name: \"{}\".to_owned(), idx: Box::new({}), value: Box::new({})}}", name, idx, value),
            Self::Function{identifier, params, block} => writeln!(f, "Statement::Function{{identifier: \"{}\".to_owned(), params: vec!{:?}.iter().map(|x| x.to_owned()).collect(), block: Box::new({})}}", identifier, params, block),
            Self::Import(x) => writeln!(f, "Statement::Import(Box::new({}))", x),
            Self::Process{readfile, writefile, block} => writeln!(f, "Statement::Process{{readfile: Box::new({readfile}), readfile: Box::new({writefile}), Box::new({block})}}"),
            Self::If{condition, block} => writeln!(f, "Statement::If{{condition: Box::new({}), block: Box::new({})}}", condition, block),
            Self::IfElse{condition, if_block, else_block} => writeln!(f, "Statement::IfElse{{condition: Box::new({}), if_block: Box::new({}), else_block: Box::new({})}}", condition, if_block, else_block),
            Self::Return(x) => writeln!(f, "Statement::Return(Box::new({}))", x),
            Self::Print(x) => writeln!(f, "Statement::Print(Box::new({}))", x),
            Self::While{condition, block} => writeln!(f, "Statement::While{{condition: Box::new({}), block: Box::new({})}}", condition, block),
            Self::Loop(x) => writeln!(f, "Statement::Loop(Box::new({}))", x),
            Self::Expression(x) => writeln!(f, "Statement::Expression(Box::new({}))", x),
            Self::Line => writeln!(f, "Statement::Line"),
            _ => writeln!(f, "")
        }.unwrap();
        Ok(())
    }
}

// For printing a block of statements as Rust code
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Literal expression
    Literal(Object),
    /// Unary operations (not, unary negation)
    Unary{
        operand: Box<Expression>, 
        operator: TokenType
    },
    /// Binary operations (plus, minus, times, divided by, to the power of, with an uncertainty of, and, or, equal, not equal, etc.)
    Binary{
        operand1: Box<Expression>, 
        operand2: Box<Expression>, 
        operator: TokenType
    },
    /// Expression in parenthesis (overrides operator precedence)
    Parenthesized(Box<Expression>),
    /// Calls function, evaluates to return value of function
    FunctionCall{identifier: String, args: Vec<Box<Expression>>},
    FinderCall{identifier: String, given: HashMap<String, Expression>, to_find: String},
    /// Calls method, evaluates to return value of method
    MethodCall{object: String, method: String, args: Vec<Box<Expression>>},
    /// Gets item from given index of array
    ArrayIndex{identifier: String, index: Box<Expression>},
    /// Defines array
    Array(Vec<Box<Expression>>)
}

// Print statements to Rust code, intended for compiling std in `build.rs`
impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Self::Literal(x) => write!(f, "Expression::Literal({})", x),
            Self::Parenthesized(x) => write!(f, "Expression::Parenthesized({})", x),
            Self::Array(x) => write!(f, "Expression::Array({})", Expressions(x.clone())),
            Self::Unary{operand, operator} => write!(f, "Expression::Unary{{operand: Box::new({}), operator: {}}}", operand, operator),
            Self::Binary{operand1, operand2, operator} => write!(f, "Expression::Binary{{operand1: Box::new({}), operand2: Box::new({}), operator: {}}}", operand1, operand2, operator),
            Self::FunctionCall{identifier, args} => write!(f, "Expression::FunctionCall{{identifier: \"{}\".to_owned(), args: vec![{}]}}", identifier, Expressions(args.clone())),
            Self::MethodCall{object, method, args} => write!(f, "Expression::FunctionCall{{object: \"{}\".to_owned(), method: \"{}\".to_owned(), args: vec![{}]}}", object, method, Expressions(args.clone())),
            Self::ArrayIndex{identifier, index} => write!(f, "Expression::ArrayIndex{{identifier: \"{}\".to_owned(), index: Box::new({})}}", identifier, index),
            _ => write!(f, "CAN'T USE FINDERS IN STD YET")
        }.unwrap();
        Ok(())
    }
}

// For displaying multiple expressions
impl Display for Expressions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut problem = false;
        for expression in self.0.iter() {
            if let Ok(_) = writeln!(f, "Box::new({}),", expression) {
                ()
            } else {
                problem = true
            }
        }
        if problem {Err(Error)} else {Ok(())}
    }
}

/// A utility struct to work around inability to `impl Display for Vec<Statement>`

pub struct Statements(pub Vec<Statement>);

/// A utility struct to work around inability to `impl Display for Vec<Box::Expression>`

pub struct Expressions(pub Vec<Box<Expression>>);