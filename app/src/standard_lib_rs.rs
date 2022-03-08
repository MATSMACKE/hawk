use hawk_lib::tree::Expression;
use std::fs::{write, read_to_string};

use crate::{Object, eval::Interpreter};

impl Interpreter {
    pub fn get_std_rs_fn(&mut self, identifier: String, args: Vec<Box<Expression>>) -> Option<Object> {
        match identifier.as_str() {
            "readfilestr" => {
                if let Object::String(file) = self.eval_expression(args[0].clone()) {
                    if let Ok(str) = read_to_string(file) {
                        Some(Object::String(str))
                    } else {
                        Some(Object::Null)
                    }
                } else {
                    Some(Object::Null)
                }
            },
            "writefile" => {
                let val = self.eval_expression(args[1].clone());
                let file = self.eval_expression(args[0].clone());
                if let Object::String(str) = val {
                    if let Object::String(filename) = file {
                        if let Ok(()) = write(filename, str) {
                            
                        } else {
                            panic!("Couldn't write file");
                        }
                    } else {
                        panic!("Incorrect filename");
                    }
                }
                Some(Object::Null)
            }
            _ => None
        }
    }
}