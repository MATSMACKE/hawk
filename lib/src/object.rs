use std::{fmt::{Display, Formatter, Result, Error}, i128};
use crate::tree::Statement;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Int(i128),
    Float(f64),
    String(String),
    Boolean(bool),
    Uncertain{value: f64, uncertainty: f64},
    Function{params: Vec<String>, block: Box<Statement>},   
    Array(Vec<Object>),
    Identifier(String)
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Self::Null => write!(f, "Object::Null"),
            Self::Int(x) => write!(f, "Object::Int({})", x),
            Self::Float(x) => write!(f, "Object::Float({})", x),
            Self::String(x) => write!(f, "Object::String(\"{}\".to_string())", x),
            Self::Boolean(x) => write!(f, "Object::Boolean({})", x),
            Self::Uncertain{value, uncertainty} => write!(f, "Object::Uncertain{{value: {}, uncertainty: {}}}", value, uncertainty),
            Self::Function{params, block} => {
                if params.len() != 0 {write!(f, "Object::Function{{params: vec!{:?}.iter().map(|x| x.to_string()).collect(), block: Box::new({})}}", params, block)}
                else {write!(f, "Object::Function{{params: Vec::new(), block: Box::new({})}}", block)}},
            Self::Array(x) => write!(f, "Object::Array({})", Objects(x.clone())),
            Self::Identifier(x) => write!(f, "Object::Identifier(\"{}\".to_string())", x)
        }
    }
}

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

pub struct Objects(Vec<Object>);