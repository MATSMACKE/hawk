use std::fs::{read_to_string, write};

use crate::error::exit;

use unicode_segmentation::UnicodeSegmentation;

use crate::object::UserPrintObject;

use hawk_common::object::Object;
use hawk_common::token::{Token, TokenType};

pub fn csv_to_datatable(filename: String, line: usize) -> Object {
    if let Ok(csvfile) = read_to_string(&filename) {
        let tokens = Lexer::lex(csvfile.as_str());
        parse_csv(tokens, line)
    } else {
        exit(&format!("Couldn't read file: {}", filename), line)
    }
}

fn parse_csv(tokens: Vec<Token>, line: usize) -> Object {
    let (i, titles) = parse_titles(&tokens, line);

    let values = parse_values(&tokens, i);

    let columns = values_to_columns(&titles, values);
    
    Object::DataTable{names: titles, data: columns}
}

fn values_to_columns(titles: &Vec<String>, values: Vec<Vec<Object>>) -> Vec<Object> {
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
    columns
}

fn parse_values(tokens: &Vec<Token>, mut i: usize) -> Vec<Vec<Object>> {
    let mut values = Vec::new();
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

    values
}

fn parse_titles(tokens: &Vec<Token>, line: usize) -> (usize, Vec<String>) {
    let mut titles: Vec<String> = Vec::new();
    let mut i = 0;

    while tokens[i].token_type != TokenType::NewLine {
        if let Some(Object::String(title)) = tokens[i].literal.clone() {
            titles.push(title)
        } else if tokens[i].token_type == TokenType::Comma {
            
        } else if let Some(x) = tokens[i].literal.clone() {
            exit(&format!("Expected identifier as title of CSV column, found {}", x), line);
        } else {
            exit("Expected a literal, found None (there's probably 2 commas without a value in between in your CSV)", line);
        }

        i += 1
    }

    i += 1;

    (i, titles)
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
            self.match_char(c);
        }

        self.add_token(TokenType::EOF, None);
    }

    fn match_char(&mut self, c: &str) {
        match c {
            " " | "\r" | "\t" => {},
            "\n" => {
                self.line = self.line + 1;
                self.add_token(TokenType::NewLine, None)
            },
            "," => self.add_token(TokenType::Comma, None),
            _ => {
                if let Ok(_) = c.parse::<usize>() {
                    self.parse_number()
                }

                if let Some(c) = c.chars().nth(0) {
                    self.column_name(c);
                }
            }
        }
    }

    fn column_name(&mut self, c: char) {
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

    fn parse_number(&mut self) {
        let mut is_float = false;

        let mut int: usize = 0;

        while let Ok(num) = self.characters[self.index - 1].parse::<usize>() {
            (is_float, int) = self.parse_int(is_float, int, num);
        }

        if is_float {   // Get decimal part of number
            self.parse_float(int);
        } else {
            self.add_token(TokenType::Int, Some(Object::Int(int as i128)));
            self.index = self.index - 1
        }
    }

    fn parse_int(&mut self, mut is_float: bool, mut int: usize, num: usize) -> (bool, usize) {
        if self.characters[self.index] == "." {
            if let Ok(_) = self.characters[self.index + 1].parse::<usize>() {
                is_float = true;
            }
        }
        int = int * 10 + num;

        self.consume_char();

        (is_float, int)
    }

    fn parse_float(&mut self, int: usize) {
        let mut decimal_digits: Vec<usize> = Vec::new();

        while let Ok(num) = self.characters[self.index].parse::<usize>() {
            decimal_digits.insert(0, num);
            self.consume_char()
        }

        let mut decimal = 0.;

        for i in decimal_digits.iter() {
            decimal = (decimal + *i as f64) / 10.;
        }

        let number = int as f64 + decimal;

        self.add_token(TokenType::Float, Some(Object::Float(number)));
    }

    fn consume_char(&mut self) {
        self.index = self.index + 1;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        self.tokens.push(Token::new(token_type, self.line, literal))
    }
}

/// Writes an `Object::DataTable` to a `.csv` file
pub fn datatable_to_csv(filename: String, datatable: Object, line: usize) {
    if let Object::DataTable{names: _, data: _} = &datatable {
        let str = datatable.format_for_csv(line);
        if let Ok(()) = write(&filename, str) {
            ()
        } else {
            exit(&format!("Couldn't write to file {}", filename), line);
        }
    } else {
        exit(&format!("Expected datatable, found {}", datatable.user_print(line)), line);
    }
}

pub trait CSV {
    fn format_for_csv(&self, line: usize) -> String;
    fn format_datatable_csv(names: Vec<String>, data: Vec<Object>, line: usize) -> String;
    fn format_datatable_csv_data(data: Vec<Object>, str: String, len: usize, line: usize) -> String;
    fn format_datatable_csv_column_names(names: Vec<String>, str: String) -> String;
}

impl CSV for Object {
    /// Generates a string representation of the Object that is suitable for a `.csv` file
    fn format_for_csv(&self, line: usize) -> String {
        match self.clone() {
            Self::Boolean(x) =>format!("{x}"),
            Self::Float(x) => format!("{x}"),
            Self::Int(x) => format!("{x}"),
            Self::String(x) => format!("{x}"),
            Self::Uncertain{value, uncertainty: _} => format!("{value}"),
            Self::DataTable{names, data} => {
                Self::format_datatable_csv(names, data, line)
            },
            _ => {
                exit(&format!("Can't write {} to CSV", self), line);
                String::new()
            }
        }
    }

    fn format_datatable_csv(names: Vec<String>, data: Vec<Object>, line: usize) -> String {
        let mut str = String::from("");
        str = Self::format_datatable_csv_column_names(names, str);
        let len: usize;
        if let Object::Column(vals) = data[0].clone() {
            len = vals.len()
        } else {
            exit(&format!("Expected column, found {}", data[0].user_print(line)), line);
            len = 0; // Unreachable
        }
        str = Self::format_datatable_csv_data(data, str, len, line);
        str
    }

    fn format_datatable_csv_data(data: Vec<Object>, mut str: String, len: usize, line: usize) -> String {
        for i in 0..len {
            str = format!("{str}\n");
            for (idx, column) in data.iter().enumerate() {
                if let Object::Column(vals) = column {
                    if idx < data.len() - 1 {
                        str = format!("{str}{}, ", vals[i].format_for_csv(line));
                    } else {
                        str = format!("{str}{}", vals[i].format_for_csv(line));
                    }
                }
            }
        }
        str
    }

    fn format_datatable_csv_column_names(names: Vec<String>, mut str: String) -> String {
        for (idx, name) in names.iter().enumerate() {
            if idx < names.len() - 1 {
                str = format!("{str}{name}, ");
            } else {
                str = format!("{str}{name}");
            }
        }
        str
    }
}