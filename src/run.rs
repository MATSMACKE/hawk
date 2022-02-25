use crate::{lexer, token};

pub fn run(source: String) {
    let tokens: Vec<token::Token> = lexer::lex(source);

}