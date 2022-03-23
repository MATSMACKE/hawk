#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(i128),
    Float(f64),
    Symbol(String)
}