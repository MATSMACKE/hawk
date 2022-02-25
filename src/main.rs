mod utils;
pub mod lexer;
pub mod io;
mod run;
pub mod token;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_script(args[1].clone()),
        _ => io::error(io::ErrorType::CommandLineArgs, 0, "")
    }
}

fn run_script(filename: String) {
    let source = std::fs::read_to_string(filename);
    match source {
        Result::Ok(source) => {
            run::run(source)
        }
        Result::Err(_) => io::error(io::ErrorType::UnreadableFile, 0, "")
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
