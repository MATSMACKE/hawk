use std::io::{stdout, Write};

use std::iter::FromIterator;

#[allow(unused_imports)]
use crossterm::{
    execute,
    cursor::{MoveToColumn},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, *},
    style::{self, Stylize}
};

pub struct Input {
    history: Vec<String>,
    history_index: usize,
    cursor_index: usize,
    input: Vec<char>,
    entered: bool
}

impl Input {
    pub fn get_input(history: Vec<String>) -> String {
        terminal::enable_raw_mode().unwrap();

        let mut input = Input {history, history_index: 0, cursor_index: 0, input: Vec::new(), entered: false};

        input.history.insert(0, String::new());

        while !input.entered {
            let text = format!("\r>> {}", String::from_iter(input.input.iter())).green();
            execute!(stdout(), style::PrintStyledContent(text)).unwrap();
            execute!(stdout(), MoveToColumn((input.cursor_index + 4) as u16)).unwrap();
            stdout().flush().unwrap();
            input.read_char();
            if !input.entered {
                execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
            }
        }

        terminal::disable_raw_mode().unwrap();

        print!("\r");

        String::from_iter(input.input.iter())
    }
    fn read_char(&mut self) {
        if let Event::Key(KeyEvent {code, modifiers: _}) = event::read().unwrap() {
            match code {
                KeyCode::Backspace => {
                    self.history_index = 0;
                    if self.cursor_index > 0 {
                        if self.cursor_index == self.input.len() {
                            self.input.pop();
                            self.cursor_index -= 1;
                        } else {
                            self.input.remove(self.cursor_index - 1);
                            self.cursor_index -= 1;
                        }
                    }
                },
                KeyCode::Char(c) => {
                    self.history_index = 0;
                    if self.cursor_index == self.input.len() {
                        self.input.push(c)
                    } else {
                        self.input.insert(self.cursor_index, c);
                    }
                    self.cursor_index += 1
                },
                KeyCode::Left => {
                    if self.cursor_index > 0 {
                        self.cursor_index -= 1;
                    }
                },
                KeyCode::Right => {
                    if self.cursor_index < self.input.len() {
                        self.cursor_index += 1;
                    }
                },
                KeyCode::Up => {
                    if self.history_index < self.history.len() - 1 {
                        self.history_index += 1;
                        self.input = self.history[self.history_index].chars().collect::<Vec<char>>();
                        self.cursor_index = self.input.len();
                    }
                },
                KeyCode::Down => {
                    if self.history_index > 0 {
                        self.history_index -= 1;
                        self.input = self.history[self.history_index].chars().collect::<Vec<char>>();
                        self.cursor_index = self.input.len();
                    }
                },
                KeyCode::Enter => {
                    self.entered = true
                }
                _ => {}
            }
        }
    }
}

pub fn print_welcome_message() {
    print_line_equals();
    execute!(stdout(), style::PrintStyledContent("\nWelcome to Hawk REPL. Exit the REPL by running 'exit' or pressing ctrl + C.\n\n".dark_grey())).unwrap();
}

pub fn print_exit_message() {
    print_line_equals()
}

fn print_line_equals() {
    if let Some(size) = termsize::get() {
        for _ in 0..size.cols {
            execute!(stdout(), style::PrintStyledContent("=".dark_grey())).unwrap();
        }
    }
}