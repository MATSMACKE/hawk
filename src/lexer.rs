use crate::token::{Token, TokenType, Object};
use unicode_segmentation::UnicodeSegmentation;

pub struct Lexer<'a> {
    characters: Vec<&'a str>,
    num_chars: usize,
    pub tokens: Vec<Token>,
    line: usize,
    lexeme_start: usize,
    index: usize
}

impl<'a> Lexer<'a> {
    pub fn lex(source: &'a str) -> Vec<Token> {
        let mut code_lexer = Lexer {characters: source.graphemes(true).collect::<Vec<&str>>(), num_chars: 0, tokens: Vec::new(), line: 1, lexeme_start: 0, index: 0};

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

                // Plus or PlusMinus
                "+" => {
                    if self.match_next("-") {
                        self.add_token(TokenType::PlusMinus, None)
                    } else {
                        self.add_token(TokenType::Plus, None)
                    }
                },

                // Equal and assignment
                "=" => {
                    if self.match_next("=") {
                        self.add_token(TokenType::EqualEqual, None)
                    } else {
                        self.add_token(TokenType::Assign, None)
                    }
                },

                // Not and not equal
                "!" => {
                    if self.match_next("=") {
                        self.add_token(TokenType::NotEqual, None)
                    } else {
                        self.add_token(TokenType::Not, None)
                    }
                },

                // Greater than and greater than or equal
                ">" => {
                    if self.match_next("=") {
                        self.add_token(TokenType::GreaterThanEqual, None)
                    } else {
                        self.add_token(TokenType::GreaterThan, None)
                    }
                },

                // Less than and less than or equal
                "<" => {
                    if self.match_next("=") {
                        self.add_token(TokenType::LessThanEqual, None)
                    } else {
                        self.add_token(TokenType::LessThan, None)
                    }
                },

                // Comments or slashes
                "/" => {
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
                },

                // String literals
                "\"" => {
                    self.lexeme_start = self.index;
                    while (!self.peek("\"")) && self.index < self.num_chars {
                        self.consume_char();
                        if self.peek("\n") {
                            self.line = self.line + 1;
                        }
                    }
                    self.add_token(TokenType::String, Some(Object::String(self.characters[self.lexeme_start..self.index].join(""))));
                    self.consume_char()
                },
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
                            self.add_token(TokenType::Int, Some(Object::Int(int as isize)));
                            self.index = self.index - 1
                        }
                    }

                    // Keywords and identifiers
                    if let Some(c) = c.chars().nth(0) {
                        if c.is_alphabetic() {
                            let mut current_token = String::from(c);
                            while let Some(c) = self.characters[self.index].chars().nth(0) {
                                self.consume_char();
                                if c.is_alphanumeric() {
                                    current_token = current_token + &c.to_string();
                                } else {
                                    break;
                                }
                            }

                            match current_token.as_str() {
                                "if" => self.add_token(TokenType::If, None),
                                "else" => self.add_token(TokenType::Else, None),
                                "loop" => self.add_token(TokenType::Loop, None),
                                "while" => self.add_token(TokenType::While, None),
                                "for" => self.add_token(TokenType::For, None),
                                "class" => self.add_token(TokenType::Class, None),
                                "super" => self.add_token(TokenType::Super, None),
                                "this" => self.add_token(TokenType::This, None),
                                "let" => self.add_token(TokenType::Let, None),
                                "const" => self.add_token(TokenType::Const, None),
                                "null" => self.add_token(TokenType::Null, None),
                                "function" => self.add_token(TokenType::Function, None),
                                "print" => self.add_token(TokenType::Print, None),
                                "true" => self.add_token(TokenType::True, Some(Object::Boolean(true))),
                                "false" => self.add_token(TokenType::False, Some(Object::Boolean(false))),
                                _ => self.add_token(TokenType::Identifier, Some(Object::Identifier(current_token)))
                            }

                            
                        }
                    }
                }
            }
        }

        self.add_token(TokenType::EOF, None);
    }

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

    fn peek(&self, expected: &str) -> bool {
        if self.index < self.num_chars {
            self.characters[self.index] == expected
        } else {
            false
        }
    }

    fn peek_another(&self, expected: &str) -> bool {
        let at_end = self.index == self.num_chars;
        let correct_char = self.characters[self.index + 1] == expected;

        !at_end && correct_char
    }

    fn consume_char(&mut self) {
        self.index = self.index + 1;
    }

    fn consume_chars(&mut self, num: usize) {
        self.index = self.index + num;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        self.tokens.push(Token::new(token_type, self.line, literal))
    }
}
