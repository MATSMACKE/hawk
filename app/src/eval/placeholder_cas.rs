#![allow(unused)]

use hawk_common::{tree::Expression, object::Object, token::TokenType};
use hawk_cli_io::error::exit;

pub struct Equation {
    lhs: Expression,
    rhs: Expression,
    solve_for: String
}

impl Equation {
    pub fn solve_for(lhs: Expression, rhs: Expression, var: String) -> Expression {
        use Expression::*;

        match (lhs.clone(), rhs.clone()) {
            (Binary{operand1, operand2, operator}, Literal(rh)) => {
                if rhs.contains(var.to_owned()) {
                    return lhs
                }
                match operator {
                    TokenType::Asterisk => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // x * a = b
                                // return b / a
                                Expression::Binary{operand1: Box::new(rhs), operand2, operator: TokenType::Slash}
                            } else {
                                // a * x = b
                                // return b / a
                                Expression::Binary{operand1: Box::new(rhs), operand2: operand1, operator: TokenType::Slash}
                            }
                        } else {
                            exit("Cannot handle this equation", 0);
                            Expression::Literal(Object::Null)
                        }
                    },

                    TokenType::Slash => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // x / a = b
                                // return b * a
                                Expression::Binary{operand1: Box::new(rhs), operand2, operator: TokenType::Asterisk}
                            } else {
                                // a / x = b
                                // return a / b
                                Expression::Binary{operand1, operand2: Box::new(rhs), operator: TokenType::Slash}
                            }
                        } else {
                            exit("Cannot handle this equaion", 4);
                            Expression::Literal(Object::Null)
                        }
                    }
                    _ => Literal(Object::Int(0))
                }
            },

            (Literal(rh), Binary{operand1, operand2, operator}) => {
                if lhs.contains(var.to_owned()) {
                    return rhs
                }
                match operator {
                    TokenType::Asterisk => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // a = x * b
                                // return a / b
                                Expression::Binary{operand1: Box::new(lhs), operand2, operator: TokenType::Slash}
                            } else {
                                // a = b * x
                                // return a / b
                                Expression::Binary{operand1: Box::new(lhs), operand2: operand1, operator: TokenType::Slash}
                            }
                        } else {
                            exit("Cannot handle this equaion", 6);
                            Expression::Literal(Object::Null)
                        }
                    },

                    TokenType::Slash => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // a = x / b
                                // return a * b
                                Expression::Binary{operand1: Box::new(lhs), operand2, operator: TokenType::Asterisk}
                            } else {
                                // a = b / x
                                // return b / a
                                Expression::Binary{operand1: operand1, operand2: Box::new(lhs), operator: TokenType::Slash}
                            }
                        } else {
                            exit("Cannot handle this equaion", 9);
                            Expression::Literal(Object::Null)
                        }
                    }
                    _ => Literal(Object::Null)
                }
            },
            _ => Literal(Object::Int(0))
        }
    }
}

#[test]
fn basic_cas() {
    assert_eq!(
        Equation::solve_for(
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operand2: Box::new(Expression::Literal(Object::Int(2))), 
                operator: TokenType::Asterisk}, 
            Expression::Literal(Object::Int(4)), 
        String::from("x")
        ), 
        
        Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Slash
        });

    assert_eq!(
        Equation::solve_for(
            Expression::Literal(Object::Int(4)), 
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operand2: Box::new(Expression::Literal(Object::Int(2))), 
                operator: TokenType::Asterisk},
        String::from("x")
        ), 
        
        Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Slash
        });
    
    assert_eq!(
        Equation::solve_for(
            Expression::Literal(Object::Int(4)), 
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operand2: Box::new(Expression::Literal(Object::Int(2))), 
                operator: TokenType::Slash},
        String::from("x")
        ), 
        
        Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Asterisk
        });

    assert_eq!(
        Equation::solve_for(
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operand2: Box::new(Expression::Literal(Object::Int(2))), 
                operator: TokenType::Slash},
                Expression::Literal(Object::Int(4)), 
        String::from("x")
        ), 
        
        Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Asterisk
        });

    assert_eq!(
        Equation::solve_for(
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Int(2))), 
                operand2: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operator: TokenType::Slash},
                Expression::Literal(Object::Int(4)), 
        String::from("x")
        ), 
        
        Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(2))),
            operand2: Box::new(Expression::Literal(Object::Int(4))),
            operator: TokenType::Slash
        });

    assert_eq!(
        Equation::solve_for(
            Expression::Literal(Object::Int(4)), 
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Int(2))), 
                operand2: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operator: TokenType::Slash},
        String::from("x")
        ), 
        
        Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(2))),
            operand2: Box::new(Expression::Literal(Object::Int(4))),
            operator: TokenType::Slash
        });
}

pub trait GetVars {
    fn get_variables(&self) -> Vec<String>;
    fn contains(&self, var: String) -> bool;
}

impl GetVars for Expression {
    fn get_variables(&self) -> Vec<String> {
        let mut in_self: Vec<String> = Vec::new();
        match self {
            Self::Binary { operand1, operand2, operator } => {
                in_self.append(&mut operand1.get_variables().to_owned());
                in_self.append(&mut operand2.get_variables().to_owned());
            },
            Self::Literal(x) => {
                if let Object::Identifier(var) = x.to_owned() {
                    in_self.push(var)
                }
            }
            _ => {
                exit("Expression not supported in finder", 1);
            }
        }
        in_self
    }

    fn contains(&self, var: String) -> bool {
        let vars = self.get_variables();
        let mut contains = false;

        for x in vars {
            if x == var {
                contains = true
            }
        }

        contains
    }
}