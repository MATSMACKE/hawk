use crate::{lexer, parser};

pub fn run(source: String) {
    let mut code_lexer = lexer::Lexer::new(&source);

    code_lexer.lex();

    let syntax_tree = parser::Parser::parse(&code_lexer.tokens);
}