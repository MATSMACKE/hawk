use core::panic;
use std::collections::HashMap;

use crate::token::{TokenType, Object};
use crate::tree::{Expression, Statement};

/// Runs parsed code from the list of statements returned by the parser
pub struct Interpreter {
    statements: Vec<Statement>,
    variables: HashMap<String, Object>
}

impl Interpreter {
    pub fn interpret(statements: Vec<Statement>) {
        let mut interpreter = Interpreter{statements, variables: HashMap::new()};

        for index in 0..interpreter.statements.len() {
            interpreter.run_statement(index)
        }
    }

    /// Executes a given statement
    fn run_statement(&mut self, index: usize) {
        match self.statements[index].clone() {
            Statement::Print(expr) => println!("{:?}", self.eval_expression(expr)),
            Statement::Definition{name, value} => {self.variables.insert(name, self.eval_expression(value));},
            _ => {}
        }
    }

    /// Traverses an expression tree to evaluate it and return an Object
    fn eval_expression(&self, expression: Box<Expression>) -> Object {
        match *expression {
            Expression::Binary{operand1, operand2, operator} => {
                let eval_op1 = self.eval_expression(operand1);
                let eval_op2 = self.eval_expression(operand2);
    
                match operator {
                    TokenType::Plus => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Int(x + y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x as f64 + y)
                            } else {
                                panic!("Can't add this type to Int")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Float(x + y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x + y)
                            } else {
                                panic!("Can't add this type to Float")
                            }
                        } else {
                            panic!("Can't add non-numbers")
                        }
                    },
                    TokenType::Asterisk => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Int(x * y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x as f64 * y)
                            } else {
                                panic!("Can't multiply this type to Int")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Float(x * y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x * y)
                            } else {
                                panic!("Can't multiply this type to Float")
                            }
                        } else {
                            panic!("Can't multiply non-numbers")
                        }
                    },
                    TokenType::Minus => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Int(x - y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x as f64 - y)
                            } else {
                                panic!("Can't subtract this type from Int")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Float(x - y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x - y)
                            } else {
                                panic!("Can't subtract this type from Float")
                            }
                        } else {
                            panic!("Can't subtract non-numbers")
                        }
                    },
                    TokenType::Slash => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Int(x / y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x as f64 / y)
                            } else {
                                panic!("Can't divide Int by this type")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Float(x / y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x / y)
                            } else {
                                panic!("Can't divide Float by this type")
                            }
                        } else {
                            panic!("Can't divide non-numbers")
                        }
                    },
                    _ => {
                        panic!("Problem")
                    }
                }
            },
            Expression::Unary{operand, operator} => {
                let eval_op = self.eval_expression(operand);
                match operator {
                    TokenType::Minus => {
                        if let Object::Int(x) = eval_op {
                            Object::Int(-x)
                        }
                        else if let Object::Float(x) = eval_op {
                            Object::Float(-x)
                        }
                        else {
                            panic!("Expected number, found {:?}", eval_op)
                        }
                    },
                    TokenType::Not => {
                        if let Object::Boolean(x) = eval_op {
                            Object::Boolean(!x)
                        } else {
                            panic!("Expected bool, found {:?}", eval_op)
                        }
                    },
                    _ => {
                        panic!("Error: expected binary operator, instead found {:?}", operator);
                    }
                }
            },
            Expression::Literal(obj) => {
                if let Object::Identifier(name) = obj {
                    if let Some(x) = self.variables.get(&name) {
                        x.clone()
                    } else {
                        panic!("Variable {name} is not defined")
                    }
                } else {
                    obj
                }
            },
            _ => Object::Null
        }
    }
}