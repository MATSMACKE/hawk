use crate::object::Object;
use crate::token::{Token, TokenType, Tokens};
use core::panic;
use std::fs::{write, read_to_string};
use unicode_segmentation::UnicodeSegmentation;

pub fn csv_to_datatable(filename: String) -> Object {
    if let Ok(csvfile) = read_to_string(&filename) {
        let tokens = Lexer::lex(csvfile.as_str());
        println!("{}", Tokens(tokens));
    } else {
        panic!("Couldn't read file: {filename}")
    }
    Object::Null
}

struct Lexer<'a> {
    characters: Vec<&'a str>,
    num_chars: usize,
    pub tokens: Vec<Token>,
    line: usize,
    index: usize
}

impl<'a> Lexer<'a> {
    pub fn lex(source: &'a str) -> Vec<Token> {
        let mut code_lexer = Lexer {characters: source.graphemes(true).collect::<Vec<&str>>(), num_chars: 0, tokens: Vec::new(), line: 1, index: 0};

        code_lexer.lex_code();

        code_lexer.tokens
    }

    pub fn lex_code(&mut self) {
        self.characters.push(" ");

        self.num_chars = self.characters.len();

        while self.index < self.num_chars {
            let c = self.characters[self.index];
            self.consume_char();
            match c {
                " " | "\r" | "\t" => {},
                "\n" => {
                    self.line = self.line + 1;
                    self.add_token(TokenType::NewLine, None)
                },
                "," => self.add_token(TokenType::Comma, None),
                _   => {
                    // Int and float literals
                    if let Ok(_) = c.parse::<usize>() {
                        let mut is_float = false;

                        let mut int: usize = 0;
                        let mut decimal: f64 = 0.;

                        while let Ok(num) = self.characters[self.index - 1].parse::<usize>() {
                            if self.characters[self.index] == "." { // Check for .
                                if let Ok(_) = self.characters[self.index + 1].parse::<usize>() {
                                    is_float = true;
                                }
                            }
                            int = int * 10 + num;
                            
                            self.consume_char();
                        }

                        if is_float {   // Get decimal part of number
                            let mut decimal_digits: Vec<usize> = Vec::new();

                            while let Ok(num) = self.characters[self.index].parse::<usize>() {
                                decimal_digits.insert(0, num);
                                self.consume_char()
                            }

                            for i in decimal_digits.iter() {
                                decimal = (decimal + *i as f64) / 10.;
                            }

                            let number = int as f64 + decimal;

                            self.add_token(TokenType::Float, Some(Object::Float(number)));
                        } else {
                            self.add_token(TokenType::Int, Some(Object::Int(int as i128)));
                            self.index = self.index - 1
                        }
                    }

                    // Keywords and identifiers
                    if let Some(c) = c.chars().nth(0) {
                        if c.is_alphabetic() {
                            let mut current_token = String::from(c);
                            while let Some(c) = self.characters[self.index].chars().nth(0) {
                                self.consume_char();
                                if c.is_alphanumeric() || c == '_' {
                                    current_token = current_token + &c.to_string();
                                } else {
                                    self.index -= 1;
                                    break
                                }
                            }
                            self.add_token(TokenType::Identifier, Some(Object::String(current_token)))
                        }
                    }
                }
            }
        }

        self.add_token(TokenType::EOF, None);
    }

    fn consume_char(&mut self) {
        self.index = self.index + 1;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        self.tokens.push(Token::new(token_type, self.line, literal))
    }
}

/// Writes an `Object::DataTable` to a `.csv` file
pub fn datatable_to_csv(filename: String, datatable: Object) {
    if let Object::DataTable(_) = &datatable {
        let str = datatable.format_for_csv();
        if let Ok(()) = write(&filename, str) {
            ()
        } else {
            panic!("Couldn't write to file: {filename}")
        }
    } else {
        panic!("Expected DataTable, instead got {}", datatable.user_print())
    }
}

impl Object {
    /// Generates a string representation of the Object that is suitable for a `.csv` file
    pub fn format_for_csv(&self) -> String {
        match self.clone() {
            Self::Boolean(x) =>format!("{x}"),
            Self::Float(x) => format!("{x}"),
            Self::Int(x) => format!("{x}"),
            Self::String(x) => format!("{x}"),
            Self::Uncertain{value, uncertainty: _} => format!("{value}"),
            Self::DataTable(columns) => {
                let mut str = String::from("");
                for (idx, col) in columns.iter().enumerate() {
                    if let Object::Column{title, data: _} = col {
                        if idx < columns.len() - 1 {
                            str = format!("{str}{title}, ");
                        } else {
                            str = format!("{str}{title}");
                        }
                    } else {
                        panic!("Expected column, instead found {}", col.user_print())
                    }
                    
                }
                let len: usize;
                if let Object::Column{title: _, data} = columns[0].clone() {
                    len = data.len()
                } else {
                    panic!("Expected Column, instead got {}", columns[0].user_print())
                }
                for i in 0..len {
                    str = format!("{str}\n");
                    for (idx, column) in columns.iter().enumerate() {
                        if let Object::Column{title: _, data} = column {
                            if idx < columns.len() - 1 {
                                str = format!("{str}{}, ", data[i].format_for_csv());
                            } else {
                                str = format!("{str}{}", data[i].format_for_csv());
                            }
                        }
                    }
                }
                str
            },
            _ => panic!("Can't write {self} to csv")
        }
    }
}