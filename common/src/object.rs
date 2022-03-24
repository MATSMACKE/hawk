use std::{fmt::{Display, Error, Formatter, Result}, i128};

use float_cmp::approx_eq;

use crate::tree::{Statement, Expression};

/// The structure that stores literals through all stages of the interpreter (from lexing to evaluating)
#[derive(Debug, Clone)]
pub enum Object {
    /// Null object
    Null,
    /// Number stored as 128 bit integer
    Int(i128),
    /// Number stored as 64 bit float
    Float(f64),
    /// A string literal
    String(String),
    /// A boolean (given by comparison operators or the keywords `true` and `false`)
    Boolean(bool),
    /// A number stored as 64 bit float with an uncertainty (also stored as 64 bit float)
    Uncertain{value: f64, uncertainty: f64},
    /// A function object that is stored in the scope where the function is defined, with parameter
    /// names as a vector of strings and the code of the actual function as a Statement
    Function{params: Vec<String>, block: Box<Statement>},
    Finder(Vec<(Expression, Expression)>),
    /// An array of any other kind of object (types can be mixed)
    Array(Vec<Object>),
    /// An identifier, such as the name of a function or variable
    Identifier(String),
    /// A column of data in a datatable
    Column(Vec<Object>),
    /// A datatable, created by processing a CSV file
    DataTable{names: Vec<String>, data:Vec<Object>}
}

// Display object to Rust source code, used to build the standard library
impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Self::Null => write!(f, "Object::Null"),
            Self::Int(x) => write!(f, "Object::Int({})", x),
            Self::Float(x) => write!(f, "Object::Float({})", x),
            Self::String(x) => write!(f, "Object::String(\"{}\".to_owned())", x),
            Self::Boolean(x) => write!(f, "Object::Boolean({})", x),
            Self::Uncertain{value, uncertainty} => write!(f, "Object::Uncertain{{value: {}, uncertainty: {}}}", value, uncertainty),
            Self::Function{params, block} => {
                if params.len() != 0 {write!(f, "Object::Function{{params: vec!{:?}.iter().map(std::string::ToString::to_string).collect(), block: Box::new({})}}", params, block)}
                else {write!(f, "Object::Function{{params: Vec::new(), block: Box::new({})}}", block)}},
            Self::Array(x) => write!(f, "Object::Array({})", Objects(x.clone())),
            Self::Identifier(x) => write!(f, "Object::Identifier(\"{}\".to_owned())", x),
            Self::Column(data) => write!(f, "Object::Column({})", Objects(data.clone())),
            Self::DataTable{names, data} => write!(f, "Object::DataTable{{ names: vec!{:?}.iter().map(|x| x.to_owned()).collect(), data: {} }}", names, Objects(data.clone())),
            _ => write!(f, "DON'T USE FINDERS IN STD YET")
        }
    }
}

// For outputting a list of objects to Rust source code
impl Display for Objects {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut problem = false;
        for object in self.0.iter() {
            if let Ok(_) = writeln!(f, "{},", object) {
                ()
            } else {
                problem = true
            }
        }
        if problem {Err(Error)} else {Ok(())}
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use Object::*;
        match (self, other) {
            (Int(a), Int(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (Int(a), Float(b)) | (Float(b), Int(a)) => approx_eq!(f64, *a as f64, *b, ulps = 3),
            (Uncertain { value, uncertainty }, Int(i)) 
            | (Int(i), Uncertain { value, uncertainty }) => (value + uncertainty) > (*i as f64) && (value - uncertainty) < (*i as f64),
            (String(a), String(b)) => a == b,
            (Uncertain { value: v1, uncertainty: u1 }, Uncertain { value: v2, uncertainty: u2 }) => v1 + u1 > v2 - u2 && v2 + u2 > v1 - u1,
            (Array(a), Array(b)) => {
                compare_vec_obj(a, b)
            },
            (Column(a), Column(b)) => {
                compare_vec_obj(a, b)
            },
            (Null, Null) => true,
            _ => false
        }
    }
}

fn compare_vec_obj(a: &Vec<Object>, b: &Vec<Object>) -> bool {
    let mut x = true;
    for (a, b) in a.iter().zip(b) {
        if a != b {
            x = false
        }
    }
    x
}

/// A utility struct to work around inability to `impl Display for Vec<Object>`
pub struct Objects(Vec<Object>);