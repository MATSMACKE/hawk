use std::io::{stdout};

use crossterm::execute;
use crossterm::style::{Color::Red, Colors, Print, SetColors};

use crate::object::Object;

/// Exit the program with an error message. Returns Null just to appease Rust
pub fn exit<'a>(message: &'a str, line: usize) -> Object {

    execute!(
        stdout(),
        SetColors(Colors{ foreground: Some(Red), background: None}),
        Print(format!("
Error: {message}
On line: {line}")),
    ).unwrap();
    eprintln!(
"");
    std::process::exit(1);

    Object::Null
}