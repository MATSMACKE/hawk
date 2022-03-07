use std::fs::{self, DirEntry};
use std::path::Path;

use hawk_lib::*;
use hawk_lib::tree::{Statement};
use hawk_lib::object::Object;

fn main() {
    let dest_path = Path::new("./src/").join("standard_lib_hawk.rs");
    let contents = String::from("use crate::{Object, token::TokenType, tree::{Expression, Statement}};

pub fn get_std_hawk_fn(identifier: String) -> Option<Object> {
match identifier.as_str() {");
    fs::write(
        &dest_path,
        format!("{contents}{}
_ => None
}}
}}", create_match())
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

fn create_match() -> String {
    let hawk_files = fs::read_dir("../standard_lib/").unwrap();
    let mut string = String::new();
    for file in hawk_files {
        if let Ok(file) = file {
            string = format!("\n{string}\n{}", create_match_arms(file));
        }
    }
    string
}

fn create_match_arms(path: DirEntry) -> String {
    let source = fs::read_to_string(path.path()).unwrap();
    let lexed = lexer::Lexer::lex(source.as_str());
    let parsed = parser::Parser::parse(&lexed);

    let mut matcharms = String::new();

    for statement in parsed {
        if let Statement::Function{identifier, params, block} = statement {
            matcharms = format!("{matcharms}\n\n{}", create_match_arm(params, block, identifier))
        }
    }

    matcharms
}

fn create_match_arm(params: Vec<String>, block: Box<Statement>, identifier: String) -> String {
    format!("\"{identifier}\" => Some({}),", Object::Function{params, block})
}