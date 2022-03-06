use std::fs;
use std::path::Path;

use hawk_lib::*;

fn main() {
    let dest_path = Path::new("./src/").join("standard_lib.rs");
    let contents = String::from("use crate::token::{Token, TokenType};
    use crate::tree::{Statement, Expression};
    use crate::object::Object;");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}