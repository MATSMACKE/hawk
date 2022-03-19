// Common types used throughout the interpreter
use crate::token::{TokenType};
use crate::tree::Expression;
use crate::object::Object;

use crate::eval::Interpreter;

impl Interpreter {
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
                self.eval_function_call(identifier, args)
            },
            Expression::Array(exprs) => {
                self.eval_array_literal(exprs)
            },
            Expression::ArrayIndex{identifier, index} => {
                self.eval_arrayindex(identifier, index)
            }
            _ => Object::Null
        }
    }

    /// Calls function, taking into account uncertainties and columns in order to 
    fn eval_function_call(&mut self, identifier: String, args: Vec<Box<Expression>>) -> Object {
        let mut uncertain_index = 0;
        let mut has_uncertain = false;
        let mut evaled_args: Vec<Object> = Vec::new();
        let mut columns: Vec<usize> = Vec::new();

        for (index, arg) in args.iter().enumerate() {
            let arg = self.eval_expression(arg.clone());
            if let Object::Uncertain{value, uncertainty} = arg {
                if has_uncertain {
                    panic!("Functions can only have one argument with an uncertainty")
                }
                has_uncertain = true;
                uncertain_index = index;
                evaled_args.push(Object::Uncertain{value, uncertainty})
            } else if let Object::Column(vals) = arg{
                columns.push(index);
                evaled_args.push(Object::Column(vals))
            } else {
                evaled_args.push(arg)
            }
        }
        if columns.len() > 0 {
            Object::Null
        }
        else if has_uncertain {
            self.call_function_with_uncertainty(identifier, evaled_args, uncertain_index)
        } else {
            let mut evaled_args: Vec<Object> = Vec::new();
            for arg in args {
                evaled_args.push(self.eval_expression(arg))
            }
            self.call_function(identifier, evaled_args)
        }
    }

    fn eval_array_literal(&mut self, exprs: Vec<Box<Expression>>) -> Object {
        let mut vals: Vec<Object> = Vec::new();
        for expr in exprs {
            vals.push(self.eval_expression(expr));
        }
        Object::Array(vals)
    }

    /// Gets index of array
    fn eval_arrayindex(&mut self, identifier: String, index: Box<Expression>) -> Object {
        let index = self.eval_expression(index);    // Evaluate the array index

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

    /// Calls a function where one argument is an `Uncertain`, using a maximum and minimum value to find the uncertainty
    fn call_function_with_uncertainty(&mut self, identifier: String, mut evaled_args: Vec<Object>, uncertain_index: usize) -> Object {
        if let Object::Uncertain{value, uncertainty} = evaled_args[uncertain_index].clone() {
            // Change uncertain arg to `value + uncertainty` to find max
            evaled_args[uncertain_index] = Object::Float(value + uncertainty);
            let max;
            match self.call_function(identifier.clone(), evaled_args.clone()) {
                Object::Float(x) => max = x,
                Object::Int(x) => max = x as f64,
                x => panic!("Expected Float or Int, got {x}")
            }

            // Change uncertain arg to `value - uncertainty` to find min
            evaled_args[uncertain_index] = Object::Float(value - uncertainty);
            let min;
            match self.call_function(identifier.clone(), evaled_args.clone()) {
                Object::Float(x) => min = x,
                Object::Int(x) => min = x as f64,
                x => panic!("Expected Float or Int, got {x}")
            }

            // Change uncertain arg to `value` to find value
            evaled_args[uncertain_index] = Object::Float(value);
            let val;
            match self.call_function(identifier, evaled_args) {
                Object::Float(x) => val = x,
                Object::Int(x) => val = x as f64,
                x => panic!("Expected Float or Int, got {x}")
            }
            Object::Uncertain{value: val, uncertainty: ((max - min) / 2.).abs()}
        } else {
            panic!("`AAAAH why in the world is this not an uncertain that's literally impossible Rust just forced me to include this panic here don't mind me")
        }
    }
}