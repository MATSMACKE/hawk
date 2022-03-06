use std::collections::HashMap;

use crate::token::{TokenType};
use crate::tree::{Expression, Statement};
use crate::object::Object;

/// Runs parsed code from the list of statements returned by the parser
pub struct Interpreter {
    statements: Vec<Statement>,
    globals: HashMap<String, Object>,
    scopes: Vec<HashMap<String, Object>>,
    loops: usize
}

impl Interpreter {
    pub fn interpret(statements: Vec<Statement>, global_state: HashMap<String, Object>) -> HashMap<String, Object> {
        let mut interpreter = Interpreter{statements, globals: global_state, loops: 0, scopes: Vec::new()};

        for index in 0..interpreter.statements.len() {
            interpreter.run_statement(interpreter.statements[index].clone())
        }

        interpreter.globals
    }

    /// Executes a given statement
    fn run_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Print(expr) => println!("{:?}", self.eval_expression(expr)),
            Statement::Definition{name, value} => {
                let val = self.eval_expression(value);
                self.globals.insert(name, val);
            },
            Statement::While{condition, block} => {
                let is_true = true;
                self.loops += 1;
                let current_loop = self.loops;
                while is_true {
                    if let Object::Boolean(is_true) = self.eval_expression(condition.clone()) {
                        if is_true && (self.loops == current_loop) {
                            self.run_statement(*block.clone())
                        }
                        else {
                            if !(self.loops == current_loop) {
                                break
                            } else {
                                self.loops -= 1;
                                break
                            }
                        }
                    } else {
                        panic!("Expected boolean as condition for if statement")
                    }
                }
            },
            Statement::Loop(block) => {
                self.loops += 1;
                let current_loop = self.loops;
                loop {
                    if self.loops == current_loop {
                        self.run_statement(*block.clone())
                    }
                    else {
                        break
                    }
                }
            },
            Statement::Break => {self.loops -= 1},
            Statement::If{condition, block} => {
                if let Object::Boolean(condition) = self.eval_expression(condition) {
                    if condition {
                        self.run_statement(*block)
                    }
                } else {
                    panic!("Expected boolean as condition for if statement")
                }
            },
            Statement::IfElse{condition, if_block, else_block} => {
                if let Object::Boolean(condition) = self.eval_expression(condition) {
                    if condition {
                        self.run_statement(*if_block)
                    } else {
                        self.run_statement(*else_block)
                    }
                } else {
                    panic!("Expected boolean as condition for if statement")
                }
            },
            Statement::Block(block) => {
                for statement in block {
                    self.run_statement(statement)
                }
            },
            Statement::Function{identifier, params, block} => {self.globals.insert(identifier, Object::Function{params, block});},
            Statement::Return(expr) => {
                let val = self.eval_expression(expr);
                self.insert_top_scope(String::from("return"), val)
            },
            _ => {}
        }
    }

    /// Traverses an expression tree to evaluate it and return an Object
    fn eval_expression(&mut self, expression: Box<Expression>) -> Object {
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
                    TokenType::EqualEqual => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x == y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean((x as f64) == y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x == y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean(x == y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::NotEqual => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x != y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean((x as f64) != y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x != y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean(x != y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::Or => {
                        if let Object::Boolean(op1) = eval_op1 {
                            if let Object::Boolean(op2) = eval_op2 {
                                Object::Boolean(op1 || op2)
                            }
                            else {
                                panic!("Logical operations can only be performed on booleans")
                            }
                        }
                        else {
                            panic!("Logical operations can only be performed on booleans")
                        }
                    },
                    TokenType::And => {
                        if let Object::Boolean(op1) = eval_op1 {
                            if let Object::Boolean(op2) = eval_op2 {
                                Object::Boolean(op1 && op2)
                            }
                            else {
                                panic!("Logical operations can only be performed on booleans")
                            }
                        }
                        else {
                            panic!("Logical operations can only be performed on booleans")
                        }
                    },
                    TokenType::LessThan => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x < y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean((x as f64) < y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x < y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean(x < y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::LessThanEqual => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x <= y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean((x as f64) <= y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x <= y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean(x <= y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::GreaterThan => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x > y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean((x as f64) > y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x > y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean(x > y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::GreaterThanEqual => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x >= y)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean((x as f64) >= y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Boolean(x >= y as f64)
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Boolean(x >= y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
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
                if let Object::Identifier(identifier) = obj {
                    self.get_variable(identifier)
                } else {
                    obj
                }
            },
            Expression::FunctionCall{identifier, args} => {
                self.scopes.push(HashMap::new());
                if let Object::Function{params, block} = self.get_variable(identifier) {
                    for (index, param) in params.iter().enumerate() {
                        let val = self.eval_expression(args[index].clone());
                        self.insert_top_scope(param.clone(), val)
                    }
                    self.run_statement(*block);
                    let result = self.get_variable(String::from("return"));
                    self.scopes.pop();
                    result
                } else {
                    panic!("can only call functions")
                }
            }
            _ => Object::Null
        }
    }

    fn get_variable(&self, identifier: String) -> Object {
        let mut index = self.scopes.len() as isize - 1;
        while index >= 0 {
            if let Some(x) = self.scopes[index as usize].get(&identifier) {
                return x.clone()
            }
            index -= 1
        }
        if let Some(x) = self.globals.get(&identifier) {
            x.clone()
        } else {
            panic!("variable {identifier} is not defined")
        }
    }

    fn insert_top_scope(&mut self, identifier: String, value: Object) {
        let index = self.scopes.len() - 1;
        self.scopes[index].insert(identifier, value);
    }
}