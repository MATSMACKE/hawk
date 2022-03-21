use std::collections::HashMap;

// Common types used throughout the interpreter
use crate::object::Object;
use crate::tree::Statement;

/// Runs parsed code from the list of statements returned by the parser
pub struct Interpreter {
    /// Parsed code to execute
    pub statements: Vec<Statement>,
    /// Stores variables in the global scope
    pub globals: HashMap<String, Object>,
    /// A stack variables in local scopes are stored
    pub scopes: Vec<HashMap<String, Object>>,
    /// Contains number of nested loops in order to handle `break`
    pub loops: usize,
    /// Current line number (updated by `Line` statement)
    pub line: usize,
}

impl Interpreter {
    /// Create an `Interpreter` and run code. `global_state` is used to store the state of the REPL.
    pub fn interpret(
        statements: Vec<Statement>,
        global_state: HashMap<String, Object>,
    ) -> HashMap<String, Object> {
        let mut interpreter = Interpreter {
            statements,
            globals: global_state,
            loops: 0,
            scopes: Vec::new(),
            line: 1,
        };

        for index in 0..interpreter.statements.len() {
            interpreter.run_statement(interpreter.statements[index].clone())
        }

        interpreter.globals
    }
}
