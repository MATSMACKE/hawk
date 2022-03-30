#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(i128),
    Decimal(f64),
    Symbol(String)
}