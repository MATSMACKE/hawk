use std::collections::HashMap;

use crate::token::{TokenType};
use crate::tree::{Expression, Statement};
use crate::object::Object;
use crate::run_script;

/// Runs parsed code from the list of statements returned by the parser
pub struct Interpreter {
    statements: Vec<Statement>,
    globals: HashMap<String, Object>,
    scopes: Vec<HashMap<String, Object>>,
    loops: usize,
    function_flag: bool,
}

impl Interpreter {
    pub fn interpret(statements: Vec<Statement>, global_state: HashMap<String, Object>) -> HashMap<String, Object> {
        let mut interpreter = Interpreter{statements, globals: global_state, loops: 0, scopes: Vec::new(), function_flag: false};

        for index in 0..interpreter.statements.len() {
            interpreter.run_statement(interpreter.statements[index].clone())
        }

        interpreter.globals
    }

    /// Executes a given statement
    pub fn run_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Print(expr) => println!("{:?}", self.eval_expression(expr)),
            Statement::Definition{name, value} => {
                let val = self.eval_expression(value);
                self.insert_top_scope(name, val)
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
            Statement::Function{identifier, params, block} => {self.insert_top_scope(identifier, Object::Function{params, block});},
            Statement::Return(expr) => {
                let val = self.eval_expression(expr);
                self.insert_top_scope(String::from("return"), val)
            },
            Statement::Import(expr) => {
                if let Object::String(filename) = self.eval_expression(expr) {
                    self.globals = run_script(filename, self.globals.clone());
                } else {
                    panic!("Expected filename to be a string")
                }
            },
            Statement::Expression(expr) => {self.eval_expression(expr);},
            _ => {}
        }
    }

    /// Traverses an expression tree to evaluate it and return an Object
    pub fn eval_expression(&mut self, expression: Box<Expression>) -> Object {
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
                    TokenType::Caret => {
                        if let Object::Int(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Int(x.pow(y as u32))
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float((x as f64).powf(y))
                            } else {
                                panic!("Can't multiply this type to Int")
                            }
                        } else if let Object::Float(x) = eval_op1 {
                            if let Object::Int(y) = eval_op2 {
                                Object::Float(x.powf(y as f64))
                            } else if let Object::Float(y) = eval_op2 {
                                Object::Float(x.powf(y))
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
                if let Object::Function{params, block} = self.get_variable(identifier.clone()) {
                    self.scopes.push(HashMap::new());
                    for (index, param) in params.iter().enumerate() {
                        if self.function_flag {
                            break
                        }
                        let val = self.eval_expression(args[index].clone());
                        self.insert_top_scope(param.clone(), val)
                    }
                    self.run_statement(*block);
                    let result = self.get_variable(String::from("return"));
                    self.scopes.pop();
                    result
                } else {
                    let check_std = self.run_fn_std(identifier.clone(), args.clone()); // Check if function exists in standard library
                    if let Some(Object::Function{params, block}) = check_std.clone() {
                        self.globals.insert(identifier.clone(), Object::Function{params: params.clone(), block: block.clone()});
                        self.scopes.push(HashMap::new());
                        for (index, param) in params.iter().enumerate() {
                            if self.function_flag {
                                break
                            }
                            let val = self.eval_expression(args[index].clone());
                            self.insert_top_scope(param.clone(), val)
                        }
                        self.run_statement(*block);
                        let result = self.get_variable(String::from("return"));
                        self.scopes.pop();
                        result
                    } else if let Some(x) = check_std {
                        x
                    }
                    else {
                        panic!("The variable {identifier} does not appear to be a function. Did you define it? Is it in a file you haven't imported?")    
                    }
                }
            },
            Expression::Array(exprs) => {
                let mut vals: Vec<Object> = Vec::new();
                for expr in exprs {
                    vals.push(self.eval_expression(expr));
                }
                Object::Array(vals)
            },
            Expression::ArrayIndex{identifier, index} => {
                let index = self.eval_expression(index);
                if let Object::Int(index) = index {
                    if index >= 0 {
                        let array = self.get_variable(identifier);
                        if let Object::Array(array) = array {
                            array[index as usize].clone()
                        } else {
                            panic!("Can only index an array")
                        }
                    }
                    else {
                        panic!("Index must be 0 or above")
                    }
                } else {
                    panic!("Index must be an int")
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
            Object::Null
        }
    }

    fn insert_top_scope(&mut self, identifier: String, value: Object) {
        if self.scopes.len() > 0 {
            let index = self.scopes.len() - 1;
            self.scopes[index].insert(identifier, value);
        } else {
            self.globals.insert(identifier, value);
        }
    }
}