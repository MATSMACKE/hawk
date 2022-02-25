use crate::token::{Token, TokenType};

pub fn lex(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let line: usize = 0;

    for c in source.chars() {
        match c {
            '(' => tokens.push(Token::new(TokenType::ParenthesisLeft, c.to_string(), line)),
            ')' => tokens.push(Token::new(TokenType::ParenthesisRight, c.to_string(), line)),
            '{' => tokens.push(Token::new(TokenType::BraceLeft, c.to_string(), line)),
            '}' => tokens.push(Token::new(TokenType::BraceRight, c.to_string(), line)),
            '[' => tokens.push(Token::new(TokenType::BracketLeft, c.to_string(), line)),
            ']' => tokens.push(Token::new(TokenType::BracketRight, c.to_string(), line)),
            '|' => tokens.push(Token::new(TokenType::Abs, c.to_string(), line)),
            '*' => tokens.push(Token::new(TokenType::Asterisk, c.to_string(), line)),
            '^' => tokens.push(Token::new(TokenType::Caret, c.to_string(), line)),
            ':' => tokens.push(Token::new(TokenType::Colon, c.to_string(), line)),
            ',' => tokens.push(Token::new(TokenType::Comma, c.to_string(), line)),
            '.' => tokens.push(Token::new(TokenType::Dot, c.to_string(), line)),
            '-' => tokens.push(Token::new(TokenType::Minus, c.to_string(), line)),
            '+' => tokens.push(Token::new(TokenType::Plus, c.to_string(), line)),
            '±' => tokens.push(Token::new(TokenType::PlusMinus, c.to_string(), line)),
            '?' => tokens.push(Token::new(TokenType::QuestionMark, c.to_string(), line)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, c.to_string(), line)),
            _   => {

            }
        }
    }

    println!("{:?}", &tokens);

    vec![]
}