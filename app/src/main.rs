use std::{collections::HashMap, env};

use hawk_cli_io::error::error;
// Import lib crate
pub use hawk_lib::*;
pub use hawk_common::*;
pub use hawk_cli_io::*;

// Used extremely often, so brought into scope
use hawk_common::object::Object;

mod run;
mod utils;

// Needs to execute from run.rs
pub mod eval;

pub mod standard_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // 1 argument, that being the name of the application
        1 => repl(),

        // 2 arguments: the name of the application and the name of the file to run
        2 => {
            run_script(args[1].clone(), HashMap::new());
        }

        // Expect either 1 argument for REPL or 2 for executing a file
        _ => {
            error(
                "Incorrect args: expected either:
No arguments (open REPL) or
1 Argument (run a .hawk file)", 0
            );
            std::process::exit(1)
        }
    }
}

/// Runs Hawk code from a file given by `filename`, returning the global scope after execution
fn run_script(filename: String, global_state: HashMap<String, Object>) -> HashMap<String, Object> {
    let source = std::fs::read_to_string(filename.clone());
    match source {
        Result::Ok(source) => {
            match run::run(source, global_state, false) {
                Ok(globals) => {globals},
                Err((message, line)) => {
                    error(&message, line);
                    std::process::exit(1)
                }
            }
        },
        Result::Err(_) => {
            error("Couldn't read file {filename}", 0);
            std::process::exit(1)
        }
    }
}

/// Manages the Hawk REPL
fn repl() {
    // Global state that will be kept throughout the REPL session
    let mut state: HashMap<String, Object> = HashMap::new();

    hawk_cli_io::shell::print_welcome_message();

    let mut history: Vec<String> = Vec::new();

    loop {
        let line = hawk_cli_io::shell::Input::get_input(history.clone());
        history.insert(0, line.to_owned());

        println!();

        if line == "exit" {
            break;
        } else {
            let result = run::run(line, state.clone(), true);
            match result {
                Ok(result) => state = result,
                Err((message, _)) => {
                    error(&message, 0);
                }
            }
        }

        println!();
    }

    hawk_cli_io::shell::print_exit_message();
}

#[cfg(test)]
mod test;
