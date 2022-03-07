use std::fmt::{Display, Formatter, Result, Error};

use crate::object::Object;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Differentiate between types of tokens
pub enum TokenType {
    // Keywords
    If,
    Else,
    Loop,
    While,
    For,
    Break,
    Class,
    Super,
    This,
    Let,
    Const,
    Null,
    Function,
    Return,
    Print,
    Import,

    // Literals
    Identifier,
    String,
    Int,
    Float,
    True,
    False,

    // Punctuation
    ParenthesisLeft,
    ParenthesisRight,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Asterisk,
    Colon,
    Caret,
    E,
    Abs,
    PlusMinus,
    QuestionMark,
    ExclamationMark,
    Assign,
    EqualEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    NotEqual,
    And,
    Or,
    Not,
    EOF
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TokenType::{:?}", self)
    }
}

#[derive(Debug, Clone)]
/// Holds a token as recognized by the scanner.
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub literal: Option<Object>
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, literal: Option<Object>) -> Self {
        Token {token_type, line, literal}
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Token: {:?} {:?}", self.token_type, self.literal)
    }
}

pub struct Tokens(pub Vec<Token>);

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut problem = false;
        for token in self.0.iter() {
            if let Ok(_) = writeln!(f, "{}", token) {
                ()
            } else {
                problem = true
            }
        }
        if problem {Err(Error)} else {Ok(())}
    }
}

