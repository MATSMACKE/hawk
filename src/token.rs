use std::str::Lines;

#[derive(Debug)]
/// Differentiate between types of tokens
pub enum TokenType {
    // Keywords
    If,
    Else,
    Loop,
    While,
    For,
    Class,
    Super,
    This,
    Let,
    Const,
    Null,
    Function,
    Print,

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
    EOF
}


#[derive(Debug)]
/// Holds a token as recognized by the scanner.
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {token_type, lexeme: lexeme, line}
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Token: {:?} {} on line {}", self.token_type, self.lexeme, self.line)
    }
}