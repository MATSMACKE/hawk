use std::{fmt::{Display, Error, Formatter, self}, i128};

use float_cmp::approx_eq;

use decimal::d128;

use term_table::row::Row;
use term_table::{Table, TableStyle};

use crate::tree::{Statement, Expression};

/// The structure that stores literals through all stages of the interpreter (from lexing to evaluating)
#[derive(Debug, Clone)]
pub enum Object {
    /// Null object
    Null,
    /// Number stored as 128 bit integer
    Int(i128),
    Decimal(d128),
    /// A string literal
    String(String),
    /// A boolean (given by comparison operators or the keywords `true` and `false`)
    Boolean(bool),
    /// A number stored as 64 bit float with an uncertainty (also stored as 64 bit float)
    Uncertain{value: d128, uncertainty: d128},
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Self::Null => write!(f, "Object::Null"),
            Self::Int(x) => write!(f, "Object::Int({})", x),
            Self::Decimal(x) => write!(f, "Object::Decimal(d128!({}))", x),
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
            Self::Finder(equations) => write!(f, "Object::Finder(vec![{}])", Equations(equations.to_owned())),
        }
    }
}

// For outputting a list of objects to Rust source code
impl Display for Objects {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

// For outputting a list of objects to Rust source code
impl Display for Equations {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut problem = false;
        for eqn in self.0.iter() {
            if let Ok(_) = writeln!(f, "({}, {}),", eqn.0, eqn.1) {
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
            (Decimal(a), Decimal(b)) => a == b,
            (Int(a), Decimal(b)) | (Decimal(b), Int(a)) => &d128::from(*a as i64) == b,
            (Uncertain { value, uncertainty }, Int(i)) 
            | (Int(i), Uncertain { value, uncertainty }) => (value + uncertainty) > d128::from(*i as i64) && (value - uncertainty) < d128::from(*i as i64),
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

/// A utility struct to work around inability to `impl Display for Vec<(Expression, Expression)>`
pub struct Equations(Vec<(Expression, Expression)>);


impl Object {
    /// Nicely formatted output for displaying objects with `print`
    pub fn user_print(&self, line: usize) -> Result<String, (String, usize)> {
        match self.clone() {
            Self::Boolean(x) => Ok(format!("{x}")),
            Self::Decimal(x) => Ok(format!("{x}")),
            Self::Int(x) => Ok(format!("{x}")),
            Self::String(x) => Ok(format!("{x}")),
            Self::Identifier(x) => Ok(format!("{x}")),
            Self::Function{params, block} => Ok(format!("Function: params: {:?}, block: {block}", params)),
            Self::Array(x) => {
                Self::user_print_array(x, line)
            },
            Self::Null => Ok(String::from("Null")),
            Self::Uncertain{value, uncertainty} => Ok(format!("{value} ± {uncertainty}")),
            Self::Column(x) => {
                Self::user_print_column(x, line)
            },
            Self::DataTable{names, data} => {
                Self::user_print_datatable(names, data, line)
            },
            Self::Finder(_) => Ok(format!("finder function"))
        }
    }

    fn user_print_datatable(names: Vec<String>, data: Vec<Object>, line: usize) -> Result<String, (String, usize)> {
        let mut table = Table::new();
        table.style = TableStyle::extended();
        if let Object::Column(_) = data[0].clone() {
            let mut title_row = Vec::new();
            for name in names {
                title_row.push(name)
            }
            table.add_row(Row::new(title_row));
            for i in 0..data.len() {
                let mut row = Vec::new();
                for column in data.clone() {
                    if let Object::Column(objs) = column {
                        row.push(objs[i].user_print(line)?)
                    } else {
                        return Err((format!("Expected column found {}", column), line));
                    }
                }
                table.add_row(Row::new(row))
            }
        }
        Ok(table.render())
    }

    fn user_print_column(x: Vec<Object>, line: usize) -> Result<String, (String, usize)> {
        let mut str = String::from("[");
        for (idx, obj) in x.iter().enumerate() {
            if idx < x.len() - 1 {
                str = format!("{str}{}, ", obj.user_print(line)?);
            } else {
                str = format!("{str}{}", obj.user_print(line)?);
            }
        }
        Ok(format!("{str}]"))
    }

    fn user_print_array(x: Vec<Object>, line: usize) -> Result<String, (String, usize)> {
        let mut str = String::from("[");
        for (idx, obj) in x.iter().enumerate() {
            if idx < x.len() - 1 {
                str = format!("{str}{}, ", obj.user_print(line)?);
            } else {
                str = format!("{str}{}", obj.user_print(line)?);
            }
        }
        Ok(format!("{str}]"))
    }
}