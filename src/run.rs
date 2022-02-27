use crate::{lexer, token};

pub fn run(source: String) {
    let mut code_lexer = lexer::Lexer::new(&source);

    code_lexer.lex();
}