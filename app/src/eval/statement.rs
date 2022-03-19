use std::collections::HashMap;

use crate::eval::Interpreter;

// Common types used throughout the interpreter
use crate::object::Object;
use crate::tree::Statement;

impl Interpreter {
    /// Executes a given statement
    pub fn run_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Print(expr) => println!("{}", self.eval_expression(expr).user_print()),
            // Variable definition
            Statement::Definition { name, value } => {
                let val = self.eval_expression(value);
                self.insert_top_scope(name, val)
            }
            Statement::While { condition, block } => {
                let is_true = true;
                self.loops += 1;
                let current_loop = self.loops;
                while is_true {
                    if let Object::Boolean(is_true) = self.eval_expression(condition.clone()) {
                        if is_true && (self.loops == current_loop) {
                            self.run_statement(*block.clone())
                        } else {
                            if !(self.loops == current_loop) {
                                break;
                            } else {
                                self.loops -= 1;
                                break;
                            }
                        }
                    } else {
                        panic!("Expected boolean as condition for if statement")
                    }
                }
            }
            Statement::Loop(block) => {
                self.loops += 1;
                let current_loop = self.loops;
                loop {
                    if self.loops == current_loop {
                        self.run_statement(*block.clone())
                    } else {
                        break;
                    }
                }
            }
            Statement::Break => self.loops -= 1,
            Statement::If { condition, block } => {
                if let Object::Boolean(condition) = self.eval_expression(condition) {
                    if condition {
                        self.run_statement(*block)
                    }
                } else {
                    panic!("Expected boolean as condition for if statement")
                }
            }
            Statement::IfElse {
                condition,
                if_block,
                else_block,
            } => {
                let evaled_condition = self.eval_expression(condition);
                if let Object::Boolean(condition) = evaled_condition {
                    // Make sure condition is a boolean
                    if condition {
                        self.run_statement(*if_block) // Run code in if statement
                    } else {
                        self.run_statement(*else_block) // Run code in else statement
                    }
                } else {
                    panic!(
                        "Expected boolean as condition for if statement, instead got {}",
                        evaled_condition
                    )
                }
            }
            Statement::Block(block) => {
                for statement in block {
                    self.run_statement(statement) // Run each statement in block
                }
            }
            Statement::Function {
                identifier,
                params,
                block,
            } => {
                self.insert_top_scope(identifier, Object::Function { params, block });
                // Define function in top scope
            }
            Statement::Return(expr) => {
                let val = self.eval_expression(expr); // Evaluate returned value
                self.insert_top_scope(String::from("return"), val) // Assign to the variable `"return"` in the top scope, which the function call will see and return
            }
            Statement::Import(expr) => {
                if let Object::String(filename) = self.eval_expression(expr) {
                    self.globals = crate::run_script(filename, self.globals.clone());
                } else {
                    panic!("Expected filename to be a string")
                }
            }
            Statement::Expression(expr) => {
                self.eval_expression(expr);
            }
            Statement::Process {
                readfile,
                writefile,
                block,
            } => {
                self.scopes.push(HashMap::new());
                if let Object::String(readfile) = self.eval_expression(readfile) {
                    let datatable = hawk_lib::csv::csv_to_datatable(readfile);
                    if let Object::DataTable { names, data } = datatable.clone() {
                        self.insert_top_scope(String::from("datatable"), datatable);
                        for (index, name) in names.iter().enumerate() {
                            self.insert_top_scope(name.clone(), data[index].clone())
                        }
                    } else {
                        panic!("expected datatable")
                    }
                }
                self.run_statement(*block);
                let mut columns: Vec<String> = Vec::new();
                let mut values: Vec<Object> = Vec::new();
                for (key, value) in &self.scopes[self.scopes.len() - 1] {
                    if let Object::Column(_) = value {
                        columns.push(key.clone());
                        values.push(value.clone());
                    }
                }
                let filename = self.eval_expression(writefile);
                self.run_fn_std(
                    String::from("write"),
                    vec![
                        filename,
                        Object::DataTable {
                            names: columns,
                            data: values,
                        },
                    ],
                );
            }
            _ => {}
        }
    }

    /// Gets variable by identifier in the topmost scope where it is defined
    pub fn get_variable(&self, identifier: String) -> Object {
        let mut index = self.scopes.len() as isize - 1;
        while index >= 0 {
            if let Some(x) = self.scopes[index as usize].get(&identifier) {
                return x.clone();
            }
            index -= 1
        }
        if let Some(x) = self.globals.get(&identifier) {
            x.clone()
        } else {
            Object::Null
        }
    }

    /// Inserts a varibable into the most local scope currently available (`HashMap` on the top of the `scopes` stack)
    pub fn insert_top_scope(&mut self, identifier: String, value: Object) {
        if self.scopes.len() > 0 {
            let index = self.scopes.len() - 1;
            self.scopes[index].insert(identifier, value);
        } else {
            self.globals.insert(identifier, value);
        }
    }
}
