use crate::{lexer, parser, eval};
use crate::object::Object;
use std::collections::HashMap;
use crate::token::Tokens;

pub fn run(source: String, global_state: HashMap<String, Object>) -> HashMap<String, Object> {
    let tokens = lexer::Lexer::lex(&source);

    println!("{}", Tokens(tokens.clone()));

    let statements = parser::Parser::parse(&tokens);

    println!("{:?}", statements);

    eval::Interpreter::interpret(statements, global_state)
}