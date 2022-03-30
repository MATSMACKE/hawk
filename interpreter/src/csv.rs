use unicode_segmentation::UnicodeSegmentation;

use hawk_common::object::Object;
use hawk_common::token::{Token, TokenType};

use decimal::d128;

pub fn csv_to_datatable(filename: String, line: usize, filein_fn: fn(String) -> Result<String, String>) -> Result<Object, (String, usize)> {
    if let Ok(csvfile) = filein_fn(filename.clone()) {
        let tokens = Lexer::lex(csvfile.as_str());
        parse_csv(tokens, line)
    } else {
        Err((format!("Couldn't read file: {}", filename), line))
    }
}

fn parse_csv(tokens: Vec<Token>, line: usize) -> Result<Object, (String, usize)> {
    let (i, titles) = parse_titles(&tokens, line)?;

    let values = parse_values(&tokens, i);

    let columns = values_to_columns(&titles, values);
    
    Ok(Object::DataTable{names: titles, data: columns})
}

fn values_to_columns(titles: &Vec<String>, values: Vec<Vec<Object>>) -> Vec<Object> {
    let mut columns: Vec<Object> = Vec::new();
    let num_cols = titles.len();
    let num_rows = values.len();
    for j in 0..num_cols {
        let mut vals = Vec::new();
        for k in 0..num_rows {
            vals.push(values[k][j].clone());
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

fn parse_titles(tokens: &Vec<Token>, line: usize) -> Result<(usize, Vec<String>), (String, usize)> {
    let mut titles: Vec<String> = Vec::new();
    let mut i = 0;

    while tokens[i].token_type != TokenType::NewLine {
        if let Some(Object::String(title)) = tokens[i].literal.clone() {
            titles.push(title)
        } else if tokens[i].token_type == TokenType::Comma {
            
        } else if let Some(x) = tokens[i].literal.clone() {
            return Err((format!("Expected identifier as title of CSV column, found {}", x), line));
        } else {
            return Err(("Expected a literal, found None (there's probably 2 commas without a value in between in your CSV)".to_string(), line));
        }

        i += 1
    }

    i += 1;

    Ok((i, titles))
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
        let mut decimal: d128 = d128!(0);
        let mut decimal_digits: Vec<usize> = Vec::new();

        while let Ok(num) = self.characters[self.index].parse::<usize>() {
            decimal_digits.insert(0, num);
            self.consume_char()
        }

        for i in decimal_digits.iter() {
            decimal = (decimal + d128::from(*i as i64)) / d128!(10);
        }

        let number = d128::from(int as i64) + decimal;

        self.add_token(TokenType::Decimal, Some(Object::Decimal(number)));
    }

    fn consume_char(&mut self) {
        self.index = self.index + 1;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        self.tokens.push(Token::new(token_type, self.line, literal))
    }
}

/// Writes an `Object::DataTable` to a `.csv` file
pub fn datatable_to_csv(filename: String, datatable: Object, line: usize, fileout_fn: fn(String, String) -> Result<(), ()>) -> Result<(), (String, usize)> {
    if let Object::DataTable{names: _, data: _} = &datatable {
        let str = datatable.format_for_csv(line)?;
        if let Ok(()) = fileout_fn(filename.clone(), str) {
            ()
        } else {
            return Err((format!("Couldn't write to file {}", filename), line));
        }
    } else {
        return Err((format!("Expected datatable, found {}", datatable.user_print(line)?), line));
    }

    Ok(())
}

pub trait CSV {
    fn format_for_csv(&self, line: usize) -> Result<String, (String, usize)>;
    fn format_datatable_csv(names: Vec<String>, data: Vec<Object>, line: usize) -> Result<String, (String, usize)>;
    fn format_datatable_csv_data(data: Vec<Object>, str: String, len: usize, line: usize) -> Result<String, (String, usize)>;
    fn format_datatable_csv_column_names(names: Vec<String>, str: String) -> Result<String, (String, usize)>;
}

impl CSV for Object {
    /// Generates a string representation of the Object that is suitable for a `.csv` file
    fn format_for_csv(&self, line: usize) -> Result<String, (String, usize)> {
        match self.clone() {
            Self::Boolean(x) => Ok(format!("{x}")),
            Self::Decimal(x) => Ok(format!("{x}")),
            Self::Int(x) => Ok(format!("{x}")),
            Self::String(x) => Ok(format!("{x}")),
            Self::Uncertain{value, uncertainty: _} => Ok(format!("{value}")),
            Self::DataTable{names, data} => {
                Self::format_datatable_csv(names, data, line)
            },
            _ => {
                Err((format!("Can't write {} to CSV", self), line))
            }
        }
    }

    fn format_datatable_csv(names: Vec<String>, data: Vec<Object>, line: usize) -> Result<String, (String, usize)> {
        let mut str = String::from("");

        let mut names = names.clone();

        let mut new_data = data.clone();

        let len: usize;
        if let Object::Column(vals) = data[0].clone() {
            len = vals.len()
        } else {
            return Err((format!("Expected column, found {}", data[0].user_print(line)?), line))
        }

        let mut added_columns = 0;

        for (i, col) in data.iter().enumerate() {
            if let Object::Column(a) = col {
                if let Object::Uncertain{value: _, uncertainty: _} = a[0] {
                    names.insert(i + added_columns + 1, format!("uncertainty{}", names[i + added_columns]));

                    let mut vals = Vec::new();
                    let mut uncerts = Vec::new();
                    for obj in a {
                        if let Object::Uncertain{value, uncertainty} = obj {
                            vals.push(Object::Decimal(*value));
                            uncerts.push(Object::Decimal(*uncertainty));   
                        }
                    }
                    new_data[i + added_columns] = Object::Column(vals);
                    new_data.insert(i + 1 + added_columns, Object::Column(uncerts));

                    added_columns += 1;
                }
            } else {
                return Err((format!("Expected column, found {}", data[0].user_print(line)?), line));
            }
        }

        str = Self::format_datatable_csv_column_names(names, str)?;
        str = Self::format_datatable_csv_data(new_data, str, len, line)?;
        Ok(str)
    }

    fn format_datatable_csv_data(data: Vec<Object>, mut str: String, len: usize, line: usize) -> Result<String, (String, usize)> {
        for i in 0..len {
            str = format!("{str}\n");
            for (idx, column) in data.iter().enumerate() {
                if let Object::Column(vals) = column {
                    if idx < data.len() - 1 {
                        str = format!("{str}{}, ", vals[i].format_for_csv(line)?);
                    } else {
                        str = format!("{str}{}", vals[i].format_for_csv(line)?);
                    }
                }
            }
        }
        Ok(str)
    }

    fn format_datatable_csv_column_names(names: Vec<String>, mut str: String) -> Result<String, (String, usize)> {
        for (idx, name) in names.iter().enumerate() {
            if idx < names.len() - 1 {
                str = format!("{str}{name}, ");
            } else {
                str = format!("{str}{name}");
            }
        }
        Ok(str)
    }
}