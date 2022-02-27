#[derive(Debug, Clone)]
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
    Not,
    EOF
}


#[derive(Debug, Clone)]
/// Holds a token as recognized by the scanner.
pub struct Token {
    token_type: TokenType,
    line: usize,
    literal: Option<Object>
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, literal: Option<Object>) -> Self {
        Token {token_type, line, literal}
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Token: {:?}", self.token_type)
    }
}

pub struct Tokens(pub Vec<Token>);

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for token in self.0.iter() {
            writeln!(f, "{:?} {:?}", token.token_type, token.literal);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Int(isize),
    Float(f64),
    String(String),
    Identifier(String)
}