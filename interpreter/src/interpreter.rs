use std::collections::HashMap;

// Common types used throughout the interpreter
use hawk_common::object::Object;
use hawk_common::tree::Statement;

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
    pub in_repl: bool,
    pub filein_fn: fn(String) -> Result<String, String>,
    pub fileout_fn: fn(String, String) -> Result<(), ()>,
    pub warn_fn: fn(String, usize) -> (),
    pub err_fn: fn(String, usize) -> (),
    pub output_fn: fn(String) -> (),
}

impl Interpreter {
    /// Create an `Interpreter` and run code. `global_state` is used to store the state of the REPL.
    pub fn interpret(
        statements: Vec<Statement>,
        global_state: HashMap<String, Object>,
        in_repl: bool,
        filein_fn: fn(String) -> Result<String, String>,
        fileout_fn: fn(String, String) -> Result<(), ()>,
        warn_fn: fn(String, usize) -> (),
        err_fn: fn(String, usize) -> (),
        output_fn: fn(String) -> (),
    ) -> Result<HashMap<String, Object>, (String, usize)> {
        let mut interpreter = Interpreter {
            statements,
            globals: global_state,
            loops: 0,
            scopes: Vec::new(),
            line: 1,
            in_repl,
            filein_fn,
            fileout_fn,
            warn_fn,
            err_fn,
            output_fn,
        };

        for index in 0..interpreter.statements.len() {
            interpreter.run_statement(interpreter.statements[index].clone())?
        }

        Ok(interpreter.globals)
    }
}
