use std::io::{stdout};

use crossterm::execute;
use crossterm::style::{Color::{Red, Yellow}, Colors, Print, SetColors};

use hawk_common::object::Object;

/// Exit the program with an error message. Returns Null just to appease Rust
pub fn exit<'a>(message: &'a str, line: usize) -> Object {

    execute!(
        stdout(),
        SetColors(Colors{ foreground: Some(Red), background: None}),
        Print(format!("Error on line {line}: {message}")),
    ).unwrap();
    eprintln!(
"");
    std::process::exit(1);

    // Satisfy return to shut up the compiler
    #[allow(unreachable_code)]
    Object::Null
}

/// Exit the program with an error message. Returns Null just to appease Rust
pub fn warn<'a>(message: &'a str, line: usize) -> Object {

    execute!(
        stdout(),
        SetColors(Colors{ foreground: Some(Yellow), background: None}),
        Print(format!("Warning on line {line}: {message}")),
    ).unwrap();
    eprintln!(
"");
    Object::Null
}