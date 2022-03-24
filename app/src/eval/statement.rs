use crate::error::exit;

use hawk_common::tree::Expression;
use std::collections::HashMap;

use crate::eval::Interpreter;

// Common types used throughout the interpreter
use hawk_common::object::Object;
use hawk_cli_io::object::UserPrint;
use crate::tree::Statement;

impl Interpreter {
    /// Executes a given statement
    pub fn run_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Print(expr) => {
                println!("{}", self.eval_expression(expr).user_print(self.line))
            }

            Statement::Definition { name, value } => self.run_definition(name, value),

            Statement::While { condition, block } => self.run_while(condition, block),

            Statement::Loop(block) => self.run_loop(block),
            Statement::Break => self.loops -= 1,

            Statement::If { condition, block } => self.run_if(condition, block),

            Statement::IfElse {
                condition,
                if_block,
                else_block,
            } => self.run_if_else(condition, if_block, else_block),

            Statement::Block(block) => self.run_block(block),

            Statement::Function {
                identifier,
                params,
                block,
            } => {
                // Define function in top scope
                self.insert_top_scope(identifier, Object::Function { params, block });
            }

            Statement::Finder {
                identifier,
                equations
            } => {
                // Define function in top scope
                self.insert_top_scope(identifier, Object::Finder(equations));
            }

            Statement::Return(expr) => {
                self.run_return(expr);
            }

            Statement::Import(expr) => {
                self.run_import(expr);
            }

            Statement::Expression(expr) => {
                self.eval_expression(expr);
            }

            Statement::Process {
                readfile,
                writefile,
                block,
            } => {
                self.run_process(readfile, writefile, block);
            }

            Statement::Line => self.line += 1,
            _ => {}
        }
    }

    fn run_process(
        &mut self, readfile: Box<Expression>, writefile: Box<Expression>, block: Box<Statement>,
    ) {
        self.scopes.push(HashMap::new());
        if let Object::String(readfile) = self.eval_expression(readfile) {
            self.open_datatable(readfile);
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

    fn open_datatable(&mut self, readfile: String) {
        let datatable = hawk_cli_io::csv::csv_to_datatable(readfile, self.line);

        if let Object::DataTable { names, data } = datatable.clone() {
            self.insert_top_scope(String::from("datatable"), datatable);
            for (index, name) in names.iter().enumerate() {
                // Insert each column of the CSV into the top scope, where it can be handled as a variable
                self.insert_top_scope(name.clone(), data[index].clone())
            }
        } else {
            exit(&format!("Expected datatable, found {}", datatable), self.line);
        }
    }

    fn run_import(&mut self, expr: Box<Expression>) {
        let evaled_filename = self.eval_expression(expr);
        if let Object::String(filename) = evaled_filename {
            self.globals = crate::run_script(filename, self.globals.clone());
        } else {
            exit(
                &format!("Expected filename to be a string, found {}", evaled_filename),
                self.line,
            );
        }
    }

    fn run_return(&mut self, expr: Box<Expression>) {
        let val = self.eval_expression(expr);

        // Assign to the variable `"return"` in the top scope, which the function call will see and return
        self.insert_top_scope(String::from("return"), val)
    }

    fn run_block(&mut self, block: Vec<Statement>) {
        for statement in block {
            self.run_statement(statement) // Run each statement in block
        }
    }

    fn run_if_else(
        &mut self, condition: Box<Expression>, if_block: Box<Statement>, else_block: Box<Statement>,
    ) {
        let evaled_condition = self.eval_expression(condition);

        // Make sure condition is a boolean
        if let Object::Boolean(condition) = evaled_condition {
            self.run_statement(if condition { *if_block } else { *else_block })
        } else {
            exit(
                &format!(
                    "Expected boolean as condition for if else statement, found {}",
                    evaled_condition
                ),
                self.line,
            );
        }
    }

    fn run_if(&mut self, condition: Box<Expression>, block: Box<Statement>) {
        let evaled_condition = self.eval_expression(condition);
        if let Object::Boolean(condition) = evaled_condition {
            if condition {
                self.run_statement(*block)
            }
        } else {
            exit(
                &format!(
                    "Expected boolean as condition for if statement, found {}",
                    evaled_condition
                ),
                self.line,
            );
        }
    }

    fn run_loop(&mut self, block: Box<Statement>) {
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

    fn run_while(&mut self, condition: Box<Expression>, block: Box<Statement>) {
        let is_true = true;
        self.loops += 1;
        let current_loop = self.loops;
        while is_true {
            let evaled_condition = self.eval_expression(condition.clone());
            if let Object::Boolean(is_true) = evaled_condition {
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
                exit(
                    &format!(
                        "Expected boolean as condition for while loop, found {}",
                        evaled_condition
                    ),
                    self.line,
                );
            }
        }
    }

    fn run_definition(&mut self, name: String, value: Box<Expression>) {
        let val = self.eval_expression(value);
        self.insert_top_scope(name, val)
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
