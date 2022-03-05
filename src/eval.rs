use core::panic;

use crate::token::{TokenType, Object};
use crate::tree::{Expression, Statement};

pub struct Interpreter {
    statements: Vec<Statement>
}

impl Interpreter {
    pub fn interpret(statements: Vec<Statement>) {
        let mut interpreter = Interpreter{statements};

        for index in 0..interpreter.statements.len() {
            interpreter.run_statement(index)
        }
    }

    fn run_statement(&mut self, index: usize) {
        match self.statements[index].clone() {
            Statement::Print(expr) => println!("{:?}", self.eval_expression(expr)),
            _ => {}
        }
    }

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
                obj
            },
            _ => Object::Null
        }
    }
}