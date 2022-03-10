use std::collections::HashMap;

use hawk_lib::csv::csv_to_datatable;

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
            Statement::Print(expr) => println!("{}", self.eval_expression(expr).user_print()),
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
            Statement::Process{readfile, writefile, block} => {
                self.scopes.push(HashMap::new());
                if let Object::String(readfile) = self.eval_expression(readfile) {
                    let datatable = csv_to_datatable(readfile);
                }
            },
            _ => {}
        }
    }

    /// Traverses an expression tree to evaluate it and return an Object
    pub fn eval_expression(&mut self, expression: Box<Expression>) -> Object {
        match *expression {
            Expression::Binary{operand1, operand2, operator} => {
                let operand1 = self.eval_expression(operand1);
                let operand2 = self.eval_expression(operand2);
    
                match operator {
                    TokenType::Plus => {
                        Interpreter::add(operand1, operand2)
                    },
                    TokenType::Minus => {
                        Interpreter::subtract(operand1, operand2)
                    },
                    TokenType::Asterisk => {
                        Interpreter::multiply(operand1, operand2)
                    },
                    TokenType::Slash => {
                        Interpreter::divide(operand1, operand2)
                    },
                    TokenType::Caret => {
                        Interpreter::exponent(operand1, operand2)
                    },
                    TokenType::EqualEqual => {
                        if let Object::Int(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x == y)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean((x as f64) == y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x == y as f64)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean(x == y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::NotEqual => {
                        if let Object::Int(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x != y)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean((x as f64) != y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x != y as f64)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean(x != y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::Or => {
                        if let Object::Boolean(op1) = operand1 {
                            if let Object::Boolean(op2) = operand2 {
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
                        if let Object::Boolean(op1) = operand1 {
                            if let Object::Boolean(op2) = operand2 {
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
                        if let Object::Int(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x < y)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean((x as f64) < y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x < y as f64)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean(x < y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::LessThanEqual => {
                        if let Object::Int(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x <= y)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean((x as f64) <= y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x <= y as f64)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean(x <= y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::GreaterThan => {
                        if let Object::Int(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x > y)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean((x as f64) > y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x > y as f64)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean(x > y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::GreaterThanEqual => {
                        if let Object::Int(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x >= y)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean((x as f64) >= y)
                            } else {
                                panic!("Can't compare Int to this type")
                            }
                        } else if let Object::Float(x) = operand1 {
                            if let Object::Int(y) = operand2 {
                                Object::Boolean(x >= y as f64)
                            } else if let Object::Float(y) = operand2 {
                                Object::Boolean(x >= y)
                            } else {
                                panic!("Can't compare Float to this type")
                            }
                        } else {
                            panic!("Can't compare non-numbers")
                        }
                    },
                    TokenType::PlusMinus => {
                        match operand1 {
                            Object::Int(x) => {
                                match operand2 {
                                    Object::Int(y) => Object::Uncertain{value: x as f64, uncertainty: y as f64},
                                    Object::Float(y) => Object::Uncertain{value: x as f64, uncertainty: y},
                                    _ => panic!("{operand2} can't be an uncertainty")
                                }
                            },
                            Object::Float(x) => {
                                match operand2 {
                                    Object::Int(y) => Object::Uncertain{value: x, uncertainty: y as f64},
                                    Object::Float(y) => Object::Uncertain{value: x, uncertainty: y},
                                    _ => panic!("{operand2} can't be an uncertainty")
                                }
                            },
                            _ => panic!("Can't add an uncertainty to {operand1}")
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

    fn add(augend: Object, addend: Object) -> Object {
        match augend {
            Object::Int(x) => {
                match addend {
                    Object::Int(y) => Object::Int(x + y),
                    Object::Float(y) => Object::Float(x as f64 + y),
                    Object::Uncertain{value: y, uncertainty: u} => Object::Uncertain{value: (x as f64) + y, uncertainty: u},
                    _ => panic!("Can't add Int to {}", addend)
                }
            },
            Object::Float(x) => {
                match addend {
                    Object::Int(y) => Object::Float(x + (y as f64)),
                    Object::Float(y) => Object::Float(x + y),
                    Object::Uncertain{value: y, uncertainty: u} => Object::Uncertain{value: x + y, uncertainty: u},
                    _ => panic!("Can't add Float to {}", addend)
                }
            },
            Object::Uncertain{value: x, uncertainty: u1} => {
                match addend {
                    Object::Int(y) => Object::Uncertain{value: x + (y as f64), uncertainty: u1},
                    Object::Float(y) => Object::Uncertain{value: x + y, uncertainty: u1},
                    Object::Uncertain{value: y, uncertainty: u2} => Object::Uncertain{value: x + y, uncertainty: u1 + u2},
                    _ => panic!("Can't add Uncertain to {}", addend)
                }
            },
            Object::Column(data) => {
                Object::Column(data)
            },
            Object::String(x) => {
                match addend {
                    Object::String(y) => Object::String(format!("{x}{y}")),
                    _ => panic!("Can't add String to {}", addend)
                }
            },
            _ => panic!("Can't add {augend}")
        }
    }

    fn subtract(minuend: Object, subtrahend: Object) -> Object {
        match minuend {
            Object::Int(x) => {
                match subtrahend {
                    Object::Int(y) => Object::Int(x - y),
                    Object::Float(y) => Object::Float(x as f64 - y),
                    Object::Uncertain{value: y, uncertainty: u} => Object::Uncertain{value: (x as f64) - y, uncertainty: u},
                    _ => panic!("Can't subtract {} from Int", subtrahend)
                }
            },
            Object::Float(x) => {
                match subtrahend {
                    Object::Int(y) => Object::Float(x - (y as f64)),
                    Object::Float(y) => Object::Float(x - y),
                    Object::Uncertain{value: y, uncertainty: u} => Object::Uncertain{value: x - y, uncertainty: u},
                    _ => panic!("Can't subtract {} from Float", subtrahend)
                }
            },
            Object::Uncertain{value: x, uncertainty: u1} => {
                match subtrahend {
                    Object::Int(y) => Object::Uncertain{value: x - (y as f64), uncertainty: u1},
                    Object::Float(y) => Object::Uncertain{value: x - y, uncertainty: u1},
                    Object::Uncertain{value: y, uncertainty: u2} => Object::Uncertain{value: x - y, uncertainty: u1 + u2},
                    _ => panic!("Can't subtract {} from Uncertain", subtrahend)
                }
            },
            _ => panic!("Can't subtract {minuend}")
        }
    }

    fn multiply(multiplicand: Object, multiplier: Object) -> Object {
        match multiplicand {
            Object::Int(x) => {
                match multiplier {
                    Object::Int(y) => Object::Int(x * y),
                    Object::Float(y) => Object::Float(x as f64 * y),
                    Object::Uncertain{value, uncertainty} => Object::Uncertain{value: value * (x as f64), uncertainty: uncertainty * (x as f64)},
                    _ => panic!("Can't multiply Int by {}", multiplier)
                }
            },
            Object::Float(x) => {
                match multiplier {
                    Object::Int(y) => Object::Float(x * (y as f64)),
                    Object::Float(y) => Object::Float(x * y),
                    Object::Uncertain{value, uncertainty} => Object::Uncertain{value: value * x, uncertainty: uncertainty * x},
                    _ => panic!("Can't multiply Float by {}", multiplier)
                }
            },
            Object::Uncertain{value: x, uncertainty: u1} => {
                match multiplier {
                    Object::Int(y) => Object::Uncertain{value: x * (y as f64), uncertainty: u1 * (y as f64)},
                    Object::Float(y) => Object::Uncertain{value: x * y, uncertainty: u1 * y},
                    Object::Uncertain{value: y, uncertainty: u2} => Object::Uncertain{value: x * y, uncertainty: x * y * ((u1 / x) + (u2 / y))},
                    _ => panic!("Can't multiply Uncertain by {}", multiplier)
                }
            },
            _ => panic!("Can't multiply {multiplicand}")
        }
    }

    fn divide(dividend: Object, divisor: Object) -> Object {
        match dividend {
            Object::Int(x) => {
                match divisor {
                    Object::Int(y) => Object::Int(x / y),
                    Object::Float(y) => Object::Float(x as f64 / y),
                    Object::Uncertain{value: y, uncertainty: u} => Object::Uncertain{value: (x as f64) / y, uncertainty: (x as f64) * u / (y * y)},
                    _ => panic!("Can't divide Int by {}", divisor)
                }
            },
            Object::Float(x) => {
                match divisor {
                    Object::Int(y) => Object::Float(x / (y as f64)),
                    Object::Float(y) => Object::Float(x / y),
                    Object::Uncertain{value: y, uncertainty: u} => Object::Uncertain{value: x / y, uncertainty: x * u / (y * y)},
                    _ => panic!("Can't divide Float by {}", divisor)
                }
            },
            Object::Uncertain{value: x, uncertainty: u1} => {
                match divisor {
                    Object::Int(y) => Object::Uncertain{value: x / (y as f64), uncertainty: u1 / (y as f64)},
                    Object::Float(y) => Object::Uncertain{value: x / y, uncertainty: u1 / y},
                    Object::Uncertain{value: y, uncertainty: u2} => Object::Uncertain{value: x / y, uncertainty: (x / y) * ((u1 / x) + (u2 / y))},
                    _ => panic!("Can't divide Uncertain by {}", divisor)
                }
            },
            _ => panic!("Can't divide {dividend}")
        }
    }

    fn exponent(base: Object, power: Object) -> Object {
        match base {
            Object::Int(x) => {
                match power {
                    Object::Int(y) => Object::Int(x.pow(y as u32)),
                    Object::Float(y) => Object::Float((x as f64).powf(y)),
                    _ => panic!("Can't raise Int to {}", power)
                }
            },
            Object::Float(x) => {
                match power {
                    Object::Int(y) => Object::Float(x.powf(y as f64)),
                    Object::Float(y) => Object::Float(x.powf(y)),
                    _ => panic!("Can't raise Float to {}", power)
                }
            },
            Object::Uncertain{value: x, uncertainty: u1} => {
                match power {
                    Object::Int(y) => Object::Uncertain{value: x.powf(y as f64), uncertainty: x.powf(y as f64) * (y as f64) * (u1 / x)},
                    Object::Float(y) => Object::Uncertain{value: x.powf(y), uncertainty: x.powf(y) * y * (u1 / x)},
                    _ => panic!("Can't raise Uncertain to {}", power)
                }
            },
            _ => panic!("Can't exponentiate {base}")
        }
    }
}