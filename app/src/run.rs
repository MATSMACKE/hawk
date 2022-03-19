use std::collections::HashMap;

use crate::{eval, lexer, parser};
use crate::object::Object;

//use crate::token::Tokens;

pub fn run(source: String, global_state: HashMap<String, Object>) -> HashMap<String, Object> {
    let tokens = lexer::Lexer::lex(&source);

    //println!("{}", Tokens(tokens.clone()));

    let statements = parser::Parser::parse(&tokens);

    //println!("{:?}\n", statements);

    eval::Interpreter::interpret(statements, global_state)
}