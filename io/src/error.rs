use std::io::{stdout};

use crossterm::execute;
use crossterm::style::{Color::{Red, Yellow}, Colors, Print, SetColors};

use hawk_common::object::Object;

/// Exit the program with an error message. Returns Null just to appease Rust
pub fn error(message: String, line: usize) -> Object {
    let in_repl = line == 0;

    execute!(
        stdout(),
        SetColors(Colors{ foreground: Some(Red), background: None}),
        Print(
            if !in_repl {
                format!("Error on line {line}: {message}\n")
            } else {
                format!("Error: {message}\n")
            }),
    ).unwrap();

    if !in_repl {
        std::process::exit(1);
    }

    // Satisfy return to shut up the compiler
    #[allow(unreachable_code)]
    Object::Null
}

/// Exit the program with an error message. Returns Null just to appease Rust
pub fn warn(message: String, line: usize) -> Object {
    let in_repl = line == 0;

    execute!(
        stdout(),
        SetColors(Colors{ foreground: Some(Yellow), background: None}),
        Print(if !in_repl {
            format!("Warning on line {line}: {message}\n")
        } else {
            format!("Warning: {message}\n")
        } ),
    ).unwrap();
    Object::Null
}