use crate::object::Object;
use crate::token::{Token, TokenType};
use std::fs::{write, read_to_string};
use unicode_segmentation::UnicodeSegmentation;

pub fn csv_to_datatable(filename: String) -> Object {
    if let Ok(csvfile) = read_to_string(&filename) {
        let tokens = Lexer::lex(csvfile.as_str());
        parse_csv(tokens)
    } else {
        panic!("Couldn't read file: {filename}")
    }
}

fn parse_csv(tokens: Vec<Token>) -> Object {
    let mut values: Vec<Vec<Object>> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let mut i = 0;
    while tokens[i].token_type != TokenType::NewLine {
        if let Some(Object::String(title)) = tokens[i].literal.clone() {
            titles.push(title)
        } else if tokens[i].token_type == TokenType::Comma {}
        else if let Some(x) = tokens[i].literal.clone() {
            panic!("Expected Identifier, found {}", x)
        } else {
            panic!("Expected a literal, found None (there's probably 2 commas without a value in between in your CSV)")
        }
        i += 1
    }
    i += 1;
    let mut row = 0;
    while tokens[i].token_type != TokenType::EOF {
        values.push(Vec::new());
        while tokens[i].token_type != TokenType::NewLine && tokens[i].token_type != TokenType::EOF {
            if let Some(x) = &tokens[i].literal {
                values[row].push(x.clone())
            }
            i += 1;
        }
        if tokens[i].token_type == TokenType::NewLine {
            i += 1;
        }
        row += 1;
    }
    let mut columns: Vec<Object> = Vec::new();
    let num_cols = titles.len();
    let num_rows = values.len();
    for j in 0..num_cols {
        let mut vals = Vec::new();
        for k in 0..num_rows {
            vals.push(values[j][k].clone());
        }
        columns.push(Object::Column(vals));
    }
    Object::DataTable{names: titles, data: columns}
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

        code_lexer.lex_csv();

        code_lexer.tokens
    }

    pub fn lex_csv(&mut self) {
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
    if let Object::DataTable{names: _, data: _} = &datatable {
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
            Self::DataTable{names, data} => {
                let mut str = String::from("");
                for (idx, name) in names.iter().enumerate() {
                    if idx < names.len() - 1 {
                        str = format!("{str}{name}, ");
                    } else {
                        str = format!("{str}{name}");
                    }
                    
                }
                let len: usize;
                if let Object::Column(vals) = data[0].clone() {
                    len = vals.len()
                } else {
                    panic!("Expected Column, instead got {}", data[0].user_print())
                }
                for i in 0..len {
                    str = format!("{str}\n");
                    for (idx, column) in data.iter().enumerate() {
                        if let Object::Column(vals) = column {
                            if idx < data.len() - 1 {
                                str = format!("{str}{}, ", vals[i].format_for_csv());
                            } else {
                                str = format!("{str}{}", vals[i].format_for_csv());
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