use std::collections::HashMap;

use hawk_common::object::Object;
use crate::{eval, lexer, parser};

//use crate::token::Tokens;

pub fn run(source: String, global_state: HashMap<String, Object>, in_repl: bool) -> Result<HashMap<String, Object>, (String, usize)> {
    let tokens = lexer::Lexer::lex(&source);

    //println!("{}", Tokens(tokens.clone()));

    let statements = parser::Parser::parse(&tokens);

    //println!("{:?}\n", statements);

    let result = eval::Interpreter::interpret(statements, global_state, in_repl)?;
    
    Ok(result)
}
