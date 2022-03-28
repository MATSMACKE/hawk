use std::{collections::HashMap, env};

use hawk_cli_io::error::error;
pub use hawk_common::*;
pub use hawk_cli_io::*;

// Used extremely often, so brought into scope
use hawk_common::object::Object;

mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filein_fn = |filename: String| {
        match std::fs::read_to_string(filename) {
            Ok(file) => Ok(file),
            Err(_) => Err("Unable to read file".to_string())
        }
    };
    let fileout_fn = |filename, data| {
        match std::fs::write(filename, data) {
            Ok(()) => Ok(()),
            Err(_) => Err(())
        }
    };
    let warn_fn = |message: String, line: usize| {hawk_cli_io::error::warn(message, line);};
    let err_fn = |message: String, line: usize| {hawk_cli_io::error::error(message, line);};
    let output_fn = |text| hawk_cli_io::output::output(text);
    
    match args.len() {
        // 1 argument, that being the name of the application
        1 => repl(),

        // 2 arguments: the name of the application and the name of the file to run
        2 => {
            hawk_interpreter::run::run_script(args[1].clone(), HashMap::new(), filein_fn, fileout_fn, warn_fn, err_fn, output_fn);
        }

        // Expect either 1 argument for REPL or 2 for executing a file
        _ => {
            error(
"Incorrect args: expected either:
No arguments (open REPL) or
1 Argument (run a .hawk file)".to_string(), 0
            );
            std::process::exit(1)
        }
    }
}

/// Manages the Hawk REPL
fn repl() {
    let filein_fn = |filename: String| {
        match std::fs::read_to_string(filename) {
            Ok(file) => Ok(file),
            Err(_) => Err("Unable to read file".to_string())
        }
    };
    let fileout_fn = |filename, data| {
        match std::fs::write(filename, data) {
            Ok(()) => Ok(()),
            Err(_) => Err(())
        }
    };
    let warn_fn = |message: String, line: usize| {hawk_cli_io::error::warn(message, line);};
    let err_fn = |message: String, line: usize| {hawk_cli_io::error::error(message, line);};
    let output_fn = |text| hawk_cli_io::output::output(text);

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
            let result = hawk_interpreter::run::run(line, state.clone(), true, filein_fn, fileout_fn, warn_fn, err_fn, output_fn);
            match result {
                Ok(result) => state = result,
                Err((message, _)) => {
                    error(message, 0);
                }
            }
        }

        println!();
    }

    hawk_cli_io::shell::print_exit_message();
}
