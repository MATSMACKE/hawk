use unicode_segmentation::UnicodeSegmentation;

use hawk_common::object::Object;
use hawk_common::token::{Token, TokenType};

pub struct Lexer<'a> {
    characters: Vec<&'a str>,
    num_chars: usize,
    pub tokens: Vec<Token>,
    line: usize,
    lexeme_start: usize,
    index: usize
}

impl<'a> Lexer<'a> {
    /// Constructs a lexer and lexes sources code
    pub fn lex(source: &'a str) -> Vec<Token> {
        let mut code_lexer = Lexer {characters: source.graphemes(true).collect::<Vec<&str>>(), num_chars: 0, tokens: Vec::new(), line: 1, lexeme_start: 0, index: 0};

        code_lexer.lex_code();

        code_lexer.tokens
    }

    /// Loops through characters of source code and converts to meaningful tokens
    pub fn lex_code(&mut self) {
        self.characters.push(" ");

        self.num_chars = self.characters.len();

        while self.index < self.num_chars {
            let c = self.characters[self.index];    // Get character to match on
            self.consume_char();
            self.match_character(c);
        }

        self.add_token(TokenType::EOF, None);   // Add an EOF after lexing all code
    }

    fn match_character(&mut self, c: &str) {
        match c {
            " " | "\r" | "\t" => {},    // Ignore whitespace
            "\n" => {
                self.line = self.line + 1;  // Keep track of line number
            },
            "(" => self.add_token(TokenType::ParenthesisLeft, None),
            ")" => self.add_token(TokenType::ParenthesisRight, None),
            "{" => self.add_token(TokenType::BraceLeft, None),
            "}" => self.add_token(TokenType::BraceRight, None),
            "[" => self.add_token(TokenType::BracketLeft, None),
            "]" => self.add_token(TokenType::BracketRight, None),
            "|" => self.add_token(TokenType::Abs, None),
            "*" => self.add_token(TokenType::Asterisk, None),
            "^" => self.add_token(TokenType::Caret, None),
            ":" => self.add_token(TokenType::Colon, None),
            "," => self.add_token(TokenType::Comma, None),
            "." => self.add_token(TokenType::Dot, None),
            "-" => self.add_token(TokenType::Minus, None),
            "±" => self.add_token(TokenType::PlusMinus, None),
            "?" => self.add_token(TokenType::QuestionMark, None),
            ";" => self.add_token(TokenType::Semicolon, None),

            "+" => {
                self.plus()
            },

            "=" => {
                self.equal()
            },

            "!" => {
                self.exclamation()
            },

            ">" => {
                self.greater()
            },

            "<" => {
                self.less()
            },

            "/" => {
                self.slash()
            },

            "\"" => {
                self.quote()
            },
            _ => {
                self.number_keyword_identifier(c)
            }
        }
    }

    /// Handles number literals, keywords, and identifiers
    fn number_keyword_identifier(&mut self, c: &str) {
        self.number(c);

        self.keyword_identifier(c)
    }

    /// Handles keywords and identifiers
    fn keyword_identifier(&mut self, c: &str) {
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

                self.match_keyword_identifier(current_token)
            }
        }
    }

    /// Lexes int and float literals
    fn number(&mut self, c: &str) {
        if let Ok(_) = c.parse::<usize>() {
            let (is_float, int) = self.parse_int();

            if is_float {   // Get decimal part of number
                self.parse_float(int);
            } else {
                self.add_token(TokenType::Int, Some(Object::Int(int as i128)));
                self.index = self.index - 1 // Undo final advance in case there is another token without a space in between
            }
        }
    }

    /// Parses an integer or integer part of a float
    fn parse_int(&mut self) -> (bool, usize) {
        let mut is_float = false;
        let mut int: usize = 0;

        while let Ok(num) = self.characters[self.index - 1].parse::<usize>() {
            if self.characters[self.index] == "." { // Check for .
                if let Ok(_) = self.characters[self.index + 1].parse::<usize>() {
                    is_float = true;
                }
            }
            int = int * 10 + num;

            self.consume_char();
        }
        (is_float, int)
    }

    /// Parses decimal part of a float
    fn parse_float(&mut self, int: usize) {
        let mut decimal: f64 = 0.;
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
    }

    /// Checks for keywords, otherwise adds identifier
    fn match_keyword_identifier(&mut self, current_token: String) {
        match current_token.as_str() {
            "if" => self.add_token(TokenType::If, None),
            "else" => self.add_token(TokenType::Else, None),
            "loop" => self.add_token(TokenType::Loop, None),
            "while" => self.add_token(TokenType::While, None),
            "for" => self.add_token(TokenType::For, None),
            "break" => self.add_token(TokenType::Break, None),
            "class" => self.add_token(TokenType::Class, None),
            "super" => self.add_token(TokenType::Super, None),
            "this" => self.add_token(TokenType::This, None),
            "let" => self.add_token(TokenType::Let, None),
            "null" => self.add_token(TokenType::Null, None),
            "function" => self.add_token(TokenType::Function, None),
            "return" => self.add_token(TokenType::Return, None),
            "print" => self.add_token(TokenType::Print, None),
            "true" => self.add_token(TokenType::True, Some(Object::Boolean(true))),
            "false" => self.add_token(TokenType::False, Some(Object::Boolean(false))),
            "or" => self.add_token(TokenType::Or, None),
            "and" => self.add_token(TokenType::And, None),
            "not" => self.add_token(TokenType::Not, None),
            "import" => self.add_token(TokenType::Import, None),
            "process" => self.add_token(TokenType::Process, None),
            _ => self.add_token(TokenType::Identifier, Some(Object::Identifier(current_token)))
        }
    }

    /// Lexes string literals
    fn quote(&mut self) {
        self.lexeme_start = self.index;
        while (!self.peek("\"")) && self.index < self.num_chars {
            self.consume_char();
            if self.peek("\n") {
                self.line = self.line + 1;
            }
        }
        self.add_token(TokenType::String, Some(Object::String(self.characters[self.lexeme_start..self.index].join(""))));
        self.consume_char()
    }

    /// Distinguishes division, single line comments, and multiline comments
    fn slash(&mut self) {
        if self.match_next("/") {           //Single line comment
            while (!self.peek("\n")) && self.index < self.num_chars {
                self.consume_char()
            }
        } else if self.match_next("*") {    // Multi line comment
            while !(self.peek("*") && self.peek_another("/")) && self.index < self.num_chars {
                self.consume_char()
            }
            self.consume_chars(2)
        } else {                                    // Slash
            self.add_token(TokenType::Slash, None)
        }
    }

    /// Distinguish between less than and less than or equal to
    fn less(&mut self) {
        if self.match_next("=") {
            self.add_token(TokenType::LessThanEqual, None)
        } else {
            self.add_token(TokenType::LessThan, None)
        }
    }

    /// Distinguish between greater than and greater than or equal to
    fn greater(&mut self) {
        if self.match_next("=") {
            self.add_token(TokenType::GreaterThanEqual, None)
        } else {
            self.add_token(TokenType::GreaterThan, None)
        }
    }

    /// Distinguish between not and not equal
    fn exclamation(&mut self) {
        if self.match_next("=") {
            self.add_token(TokenType::NotEqual, None)
        } else {
            self.add_token(TokenType::Not, None)
        }
    }

    /// Distinguish between equality checking and assignment
    fn equal(&mut self) {
        if self.match_next("=") {
            self.add_token(TokenType::EqualEqual, None)
        } else if self.match_next(">") {
            self.add_token(TokenType::FatArrow, None)
        } else {
            self.add_token(TokenType::Assign, None)
        }
    }

    /// Distinguish between plus and plusminus
    fn plus(&mut self) {
        if self.match_next("-") {
            self.add_token(TokenType::PlusMinus, None)
        } else {
            self.add_token(TokenType::Plus, None)
        }
    }

    /// Checks if next character matches and, if so, consumes
    fn match_next(&mut self, expected: &str) -> bool {
        let at_end = self.index == self.num_chars;
        let correct_char = self.characters[self.index] == expected;

        if !at_end && correct_char {
            self.consume_char();
            true
        }
        else {
            false
        }
    }

    /// Checks if next character is the expected character
    fn peek(&self, expected: &str) -> bool {
        if self.index < self.num_chars {
            self.characters[self.index] == expected
        } else {
            false
        }
    }

    /// Checks if character two characters ahead is the expected character
    fn peek_another(&self, expected: &str) -> bool {
        let at_end = self.index == self.num_chars;
        let correct_char = self.characters[self.index + 1] == expected;

        !at_end && correct_char
    }

    /// Advances the index by 1
    fn consume_char(&mut self) {
        self.index = self.index + 1;
    }

    /// Advances the index by `num`
    fn consume_chars(&mut self, num: usize) {
        self.index = self.index + num;
    }

    /// Adds lexed token
    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        self.tokens.push(Token::new(token_type, self.line, literal))
    }
}
