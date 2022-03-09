pub mod lexer;
pub mod object;
pub mod parser;
pub mod token;
pub mod tree;
pub mod csv;

trait RustRepr {
    fn represent(&self) -> String;
}