use crate::{lexer, parser, eval, token::Tokens};

pub fn run(source: String) {
    let tokens = lexer::Lexer::lex(&source);

    //println!("{}", Tokens(tokens.clone()));

    let statements = parser::Parser::parse(&tokens);

    //println!("{:?}", statements);

    eval::Interpreter::interpret(statements)
}