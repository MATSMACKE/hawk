pub enum TokenType {
    Keyword(Keyword),
    Literal(Literal),
    Punctuation(Punctuation)
}

pub enum Keyword {
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
    Print
}

pub enum Literal {
    Identifier,
    String,
    Int,
    Float,
    True,
    False
}

pub enum Punctuation {
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
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize
}

impl Token {
    fn new(token_type: TokenType, lexeme: &str, line: usize) -> Self {
        Token {token_type, lexeme: lexeme.to_string(), line}
    }
}