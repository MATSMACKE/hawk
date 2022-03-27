#![allow(unused)]

use hawk_common::{tree::Expression, object::Object, token::TokenType};

pub struct Equation {
    lhs: Expression,
    rhs: Expression,
    solve_for: String
}

impl Equation {
    pub fn solve_for(lhs: Expression, rhs: Expression, var: String) -> Result<Expression, (String, usize)> {
        use Expression::*;

        match (lhs.clone(), rhs.clone()) {
            (Binary{operand1, operand2, operator}, Literal(rh)) => {
                if rhs.contains(var.to_owned())? {
                    return Ok(lhs)
                }
                match operator {
                    TokenType::Asterisk => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // x * a = b
                                // return b / a
                                Ok(Expression::Binary{operand1: Box::new(rhs), operand2, operator: TokenType::Slash})
                            } else {
                                // a * x = b
                                // return b / a
                                Ok(Expression::Binary{operand1: Box::new(rhs), operand2: operand1, operator: TokenType::Slash})
                            }
                        } else {
                            Err(("Cannot handle this equation".to_string(), 0))
                        }
                    },

                    TokenType::Slash => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // x / a = b
                                // return b * a
                                Ok(Expression::Binary{operand1: Box::new(rhs), operand2, operator: TokenType::Asterisk})
                            } else {
                                // a / x = b
                                // return a / b
                                Ok(Expression::Binary{operand1, operand2: Box::new(rhs), operator: TokenType::Slash})
                            }
                        } else {
                            Err(("Cannot handle this equaion".to_string(), 4))
                        }
                    }
                    _ => Ok(Literal(Object::Int(0)))
                }
            },

            (Literal(rh), Binary{operand1, operand2, operator}) => {
                if lhs.contains(var.to_owned())? {
                    return Ok(rhs)
                }
                match operator {
                    TokenType::Asterisk => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // a = x * b
                                // return a / b
                                Ok(Expression::Binary{operand1: Box::new(lhs), operand2, operator: TokenType::Slash})
                            } else {
                                // a = b * x
                                // return a / b
                                Ok(Expression::Binary{operand1: Box::new(lhs), operand2: operand1, operator: TokenType::Slash})
                            }
                        } else {
                            Err(("Cannot handle this equaion".to_string(), 6))
                        }
                    },

                    TokenType::Slash => {
                        if let Literal(Object::Identifier(str)) = *operand1.to_owned() {
                            if str == var {
                                // a = x / b
                                // return a * b
                                Ok(Expression::Binary{operand1: Box::new(lhs), operand2, operator: TokenType::Asterisk})
                            } else {
                                // a = b / x
                                // return b / a
                                Ok(Expression::Binary{operand1: operand1, operand2: Box::new(lhs), operator: TokenType::Slash})
                            }
                        } else {
                            Err(("Cannot handle this equaion".to_string(), 9))
                        }
                    }
                    _ => Ok(Literal(Object::Null))
                }
            },
            _ => Ok(Literal(Object::Int(0)))
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
        
        Ok(Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Slash
        }));

    assert_eq!(
        Equation::solve_for(
            Expression::Literal(Object::Int(4)), 
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operand2: Box::new(Expression::Literal(Object::Int(2))), 
                operator: TokenType::Asterisk},
        String::from("x")
        ), 
        
        Ok(Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Slash
        }));
    
    assert_eq!(
        Equation::solve_for(
            Expression::Literal(Object::Int(4)), 
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operand2: Box::new(Expression::Literal(Object::Int(2))), 
                operator: TokenType::Slash},
        String::from("x")
        ), 
        
        Ok(Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Asterisk
        }));

    assert_eq!(
        Equation::solve_for(
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operand2: Box::new(Expression::Literal(Object::Int(2))), 
                operator: TokenType::Slash},
                Expression::Literal(Object::Int(4)), 
        String::from("x")
        ), 
        
        Ok(Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(4))),
            operand2: Box::new(Expression::Literal(Object::Int(2))),
            operator: TokenType::Asterisk
        }));

    assert_eq!(
        Equation::solve_for(
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Int(2))), 
                operand2: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operator: TokenType::Slash},
                Expression::Literal(Object::Int(4)), 
        String::from("x")
        ), 
        
        Ok(Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(2))),
            operand2: Box::new(Expression::Literal(Object::Int(4))),
            operator: TokenType::Slash
        }));

    assert_eq!(
        Equation::solve_for(
            Expression::Literal(Object::Int(4)), 
            Expression::Binary{
                operand1: Box::new(Expression::Literal(Object::Int(2))), 
                operand2: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), 
                operator: TokenType::Slash},
        String::from("x")
        ), 
        
        Ok(Expression::Binary{
            operand1: Box::new(Expression::Literal(Object::Int(2))),
            operand2: Box::new(Expression::Literal(Object::Int(4))),
            operator: TokenType::Slash
        }));
}

pub trait GetVars {
    fn get_variables(&self) -> Result<Vec<String>, (String, usize)> ;
    fn contains(&self, var: String) -> Result<bool, (String, usize)>;
}

impl GetVars for Expression {
    fn get_variables(&self) -> Result<Vec<String>, (String, usize)> {
        let mut in_self: Vec<String> = Vec::new();
        match self {
            Self::Binary { operand1, operand2, operator } => {
                in_self.append(&mut operand1.get_variables()?.to_owned());
                in_self.append(&mut operand2.get_variables()?.to_owned());
            },
            Self::Literal(x) => {
                if let Object::Identifier(var) = x.to_owned() {
                    in_self.push(var)
                }
            }
            _ => {
                return Err(("Expression not supported in finder".to_string(), 1));
            }
        }
        Ok(in_self)
    }

    fn contains(&self, var: String) -> Result<bool, (String, usize)> {
        let vars = self.get_variables()?;
        let mut contains = false;

        for x in vars {
            if x == var {
                contains = true
            }
        }

        Ok(contains)
    }
}