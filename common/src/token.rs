use std::fmt::{Display, Error, Formatter, Result};

use crate::object::Object;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
    Null,
    Function,
    Return,
    Print,
    Import,
    Process,
    Finder,
    Find,
    Equation,

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
    FatArrow,
    NewLine,
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

pub trait UserPrint {
    fn user_print(&self) -> String;
}

impl UserPrint for TokenType {
    fn user_print(&self) -> String {
        match self {
            TokenType::Abs => format!("|"),
            TokenType::And => format!("and"),
            TokenType::Assign => format!("="),
            TokenType::Asterisk => format!("*"),
            TokenType::BraceLeft => format!("{{"),
            TokenType::BraceRight => format!("}}"),
            TokenType::BracketLeft => format!("["),
            TokenType::BracketRight => format!("]"),
            TokenType::Break => format!("break"),
            TokenType::Caret => format!("^"),
            TokenType::Class => format!("class"),
            TokenType::Colon => format!(":"),
            TokenType::Comma => format!(","),
            TokenType::Dot => format!("."),
            TokenType::E => format!("E"),
            TokenType::EOF => format!("end of file"),
            TokenType::Else => format!("else"),
            TokenType::EqualEqual => format!("=="),
            TokenType::Equation => format!("equation"),
            TokenType::False => format!("false"),
            TokenType::FatArrow => format!("=>"),
            TokenType::Find => format!("find"),
            TokenType::Finder => format!("finder"),
            TokenType::Float => format!("Float"),
            TokenType::For => format!("for"),
            TokenType::Function => format!("function"),
            TokenType::GreaterThan => format!(">"),
            TokenType::GreaterThanEqual => format!(">="),
            TokenType::Identifier => format!("identifier"),
            TokenType::If => format!("if"),
            TokenType::Import => format!("import"),
            TokenType::Int => format!("Integer"),
            TokenType::LessThan => format!("<"),
            TokenType::LessThanEqual => format!("<="),
            TokenType::Let => format!("let"),
            TokenType::Loop => format!("loop"),
            TokenType::Minus => format!("-"),
            TokenType::NewLine => format!("newline"),
            TokenType::Not => format!("not"),
            TokenType::NotEqual => format!("!="),
            TokenType::Null => format!("null"),
            TokenType::Or => format!("or"),
            TokenType::ParenthesisLeft => format!("("),
            TokenType::ParenthesisRight => format!(")"),
            TokenType::Plus => format!("+"),
            TokenType::PlusMinus => format!("±"),
            TokenType::Print => format!("print"),
            TokenType::Process => format!("process"),
            TokenType::QuestionMark => format!("?"),
            TokenType::Return => format!("return"),
            TokenType::Semicolon => format!(";"),
            TokenType::Slash => format!("/"),
            TokenType::String => format!("String"),
            TokenType::Super => format!("super"),
            TokenType::This => format!("this"),
            TokenType::True => format!("true"),
            TokenType::While => format!("while"),
        }
    }
}