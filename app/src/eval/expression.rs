use std::collections::HashMap;

use crate::eval::Interpreter;
use crate::object::Object;

// Common types used throughout the interpreter
use crate::token::TokenType;
use crate::tree::Expression;

impl Interpreter {
    /// Traverses an expression tree to evaluate it and return an Object
    pub fn eval_expression(&mut self, expression: Box<Expression>) -> Object {
        match *expression {
            Expression::Binary{operand1, operand2, operator} => {
                self.eval_binary(operand1, operand2, operator)
            },
            Expression::Unary{operand, operator} => {
                self.eval_unary(operand, operator)
            },
            Expression::Literal(obj) => {
                self.eval_literal(obj)
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

    /// Match operator and call method to evaluate binary operation
    fn eval_binary(&mut self, operand1: Box<Expression>, operand2: Box<Expression>, operator: TokenType) -> Object {
        let operand1 = self.eval_expression(operand1);
        let operand2 = self.eval_expression(operand2);

        match operator {
            TokenType::Plus => {
                Self::add(operand1, operand2)
            },
            TokenType::Minus => {
                Self::subtract(operand1, operand2)
            },
            TokenType::Asterisk => {
                Self::multiply(operand1, operand2)
            },
            TokenType::Slash => {
                Self::divide(operand1, operand2)
            },
            TokenType::Caret => {
                Self::exponent(operand1, operand2)
            },
            TokenType::PlusMinus => {
                Self::make_uncertain(operand1, operand2)
            },
            TokenType::EqualEqual => {
                Self::equalequal(operand1, operand2)
            },
            TokenType::NotEqual => {
                Self::notequal(operand1, operand2)
            },
            TokenType::Or => {
                Self::or(operand1, operand2)
            },
            TokenType::And => {
                Self::and(operand1, operand2)
            },
            TokenType::LessThan => {
                Self::lessthan(operand1, operand2)
            },
            TokenType::LessThanEqual => {
                Self::lessthanequal(operand1, operand2)
            },
            TokenType::GreaterThan => {
                Self::greaterthan(operand1, operand2)
            },
            TokenType::GreaterThanEqual => {
                Self::greaterthanequal(operand1, operand2)
            },
            _ => {
                panic!("Problem")
            }
        }
    }

    /// Match operator and call method to evaluate unary expression
    fn eval_unary(&mut self, operand: Box<Expression>, operator: TokenType) -> Object {
        let eval_op = self.eval_expression(operand);
        match operator {
            TokenType::Minus => {
                Self::negate(eval_op)
            },
            TokenType::Not => {
                Self::not(eval_op)
            },
            _ => {
                panic!("Error: expected binary operator, instead found {:?}", operator);
            }
        }
    }

    /// Evaluates literal expression
    fn eval_literal(&mut self, obj: Object) -> Object {
        if let Object::Identifier(identifier) = obj {
            self.get_variable(identifier)   // Dereference if `obj` is an identifier
        } else {
            obj
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

    /// Turns array literal into array object
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

    pub fn call_function(&mut self, identifier: String, args: Vec<Object>) -> Object {
        if let Object::Function { params, block } = self.get_variable(identifier.clone()) {
            self.scopes.push(HashMap::new());
            for (index, param) in params.iter().enumerate() {
                let arg = args[index].clone();
                self.insert_top_scope(param.clone(), arg)
            }
            self.run_statement(*block);
            let result = self.get_variable(String::from("return"));
            self.scopes.pop();
            result
        } else {
            let check_std = self.run_fn_std(identifier.clone(), args.clone()); // Check if function exists in standard library
            if let Some(Object::Function { params, block }) = check_std.clone() {
                self.globals.insert(
                    identifier.clone(),
                    Object::Function {
                        params: params.clone(),
                        block: block.clone(),
                    },
                );
                self.scopes.push(HashMap::new());
                for (index, param) in params.iter().enumerate() {
                    let val = args[index].clone();
                    self.insert_top_scope(param.clone(), val)
                }
                self.run_statement(*block);
                let result = self.get_variable(String::from("return"));
                self.scopes.pop();
                result
            } else if let Some(x) = check_std {
                x
            } else {
                panic!("The variable {identifier} does not appear to be a function. Did you define it? Is it in a file you haven't imported?")
            }
        }
    }
}