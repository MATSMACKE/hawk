mod utils;
mod run;
pub mod eval;
// pub mod standard_lib_hawk;
// pub mod standard_lib_rs;
// pub mod standard_lib;

pub use hawk_lib::*;

use core::panic;
use std::{env, collections::HashMap};

pub mod standard_lib;
pub mod standard_lib_rs;
pub mod standard_lib_hawk;

use crate::object::Object;

fn main() {


    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => {run_script(args[1].clone(), HashMap::new());},
        _ => panic!("incorrect args")
    }
}

fn run_script(filename: String, global_state: HashMap<String, Object>) -> HashMap<String, Object> {
    let source = std::fs::read_to_string(filename);
    match source {
        Result::Ok(source) => {
            run::run(source, global_state)
        }
        Result::Err(_) => panic!("couldn't read file")
    }
}

fn repl() {
    let mut state: HashMap<String, Object> = HashMap::new();
    if let Some(size) = termsize::get() {
        for _ in 0..size.cols {
            print!("=")
        }
    }
    print!("\nWelcome to Hawk REPL. Exit the REPL by running 'exit' or pressing ctrl + C.");
    loop {
        print!("\n>> ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
        if line == "exit\n" {break}
        else {
            state = run::run(line, state)
        }
    }
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