use crate::tree::Statement;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Int(isize),
    Float(f64),
    String(String),
    Boolean(bool),
    Uncertain{value: f64, uncertainty: f64},
    Function{params: Vec<String>, block: Box<Statement>},
    Identifier(String)
}