use std::io::stdout;

use crossterm::{
    execute,
    style::{self, Stylize}
};

pub fn output(message: String) {
    execute!(stdout(), style::PrintStyledContent(message.dark_magenta())).unwrap();
    println!()
}