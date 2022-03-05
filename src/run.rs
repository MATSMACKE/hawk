use crate::{lexer, parser, eval};

pub fn run(source: String) {
    let tokens = lexer::Lexer::lex(&source);

    let statements = parser::Parser::parse(&tokens);

    eval::Interpreter::interpret(statements)
}