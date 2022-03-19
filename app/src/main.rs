mod utils;
mod run;

// Needs to execute from run.rs
pub mod eval;
pub mod eval_expr;

// Import lib crate
pub use hawk_lib::*;

use std::{env, collections::HashMap};

// The standard library
pub mod standard_lib;
pub mod standard_lib_rs;
pub mod standard_lib_hawk;

// Used extremely often, so brought into scope
use crate::object::Object;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // 1 argument, that being the name of the application
        1 => repl(),

        // 2 arguments: the name of the application and the name of the file to run
        2 => {run_script(args[1].clone(), HashMap::new());},

        // Expect either 1 argument for REPL or 2 for executing a file
        _ => panic!("incorrect args")
    }
}

/// Runs Hawk code from a file given by `filename`
fn run_script(filename: String, global_state: HashMap<String, Object>) -> HashMap<String, Object> {
    let source = std::fs::read_to_string(filename);
    match source {
        Result::Ok(source) => {
            run::run(source, global_state)
        }
        Result::Err(_) => panic!("couldn't read file")
    }
}

/// Manages the Hawk REPL
fn repl() {
    // Global state that will be kept throughout the REPL session
    let mut state: HashMap<String, Object> = HashMap::new();

    // Print line of equal signs and welcome message
    if let Some(size) = termsize::get() {
        for _ in 0..size.cols {
            print!("=")
        }
    }
    print!("\nWelcome to Hawk REPL. Exit the REPL by running 'exit' or pressing ctrl + C.");

    // Main REPL loop
    loop {
        // Output '>>' to indicate input needed
        print!("\n>> ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!"); // Force output

        // Read code from terminal
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        // Check if need to exit REPL
        if line == "exit\n" {break}

        else {
            // Run the given code
            state = run::run(line, state)
        }
    }

    // Print another line of equal signs to close off the session
    if let Some(size) = termsize::get() {
        for _ in 0..size.cols {
            print!("=")
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_lex_and_parse_expr() {

    }
}