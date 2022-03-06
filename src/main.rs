mod utils;
pub mod lexer;
mod run;
pub mod token;
pub mod tree;
pub mod parser;
pub mod eval;
pub mod object;

use core::panic;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_script(args[1].clone()),
        _ => panic!("incorrect args")
    }
}

fn run_script(filename: String) {
    let source = std::fs::read_to_string(filename);
    match source {
        Result::Ok(source) => {
            run::run(source)
        }
        Result::Err(_) => panic!("couldn't read file")
    }
}

fn repl() {
    println!("Welcome to Hawk REPL");
    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
        if line == "exit()" {break}
        else {run::run(line)}
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_lex_and_parse_expr() {

    }
}