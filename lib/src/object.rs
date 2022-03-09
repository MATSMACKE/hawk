use std::{fmt::{Display, Formatter, Result, Error}, i128};
use crate::tree::Statement;
use term_table::{
    row::Row,
    table_cell::TableCell,
};
use term_table::{Table, TableStyle};

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
    Identifier(String),
    Column{title: String, data: Vec<Object>},
    DataTable(Vec<Object>)
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
            Self::Identifier(x) => write!(f, "Object::Identifier(\"{}\".to_string())", x),
            Self::Column{title, data} => write!(f, "Object::Column(title: \"{title}\".to_string(), data: {})", Objects(data.clone())),
            Self::DataTable(x) => write!(f, "Object::DataTable({})", Objects(x.clone()))
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

impl Object {
    pub fn user_print(&self) -> String {
        match self.clone() {
            Self::Boolean(x) =>format!("{x}"),
            Self::Float(x) => format!("{x}"),
            Self::Int(x) => format!("{x}"),
            Self::String(x) => format!("{x}"),
            Self::Identifier(x) => format!("{x}"),
            Self::Function{params, block} => format!("Function: params: {:?}, block: {block}", params),
            Self::Array(x) => {
                let mut str = String::from("[");
                for (idx, obj) in x.iter().enumerate() {
                    if idx < x.len() - 1 {
                        str = format!("{str}{}, ", obj.user_print());
                    } else {
                        str = format!("{str}{}", obj.user_print());
                    }
                }
                format!("{str}]")
            },
            Self::Null => String::from("Null"),
            Self::Uncertain{value, uncertainty} => format!("{value} ± {uncertainty}"),
            Self::Column{title, data} => {
                let mut str = String::from(format!("{title}: ["));
                for (idx, obj) in data.iter().enumerate() {
                    if idx < data.len() - 1 {
                        str = format!("{str}{}, ", obj.user_print());
                    } else {
                        str = format!("{str}{}", obj.user_print());
                    }
                }
                format!("{str}]")
            },
            Self::DataTable(columns) => {
                let mut table = Table::new();
                table.style = TableStyle::extended();
                if let Object::Column{data, title: _} = columns[0].clone() {
                    let mut title_row = Vec::new();
                    for column in columns.clone() {
                        if let Object::Column{title, data: _} = column {
                            title_row.push(TableCell::new(title))
                        }
                    }
                    table.add_row(Row::new(title_row));
                    for i in 0..data.len() {
                        let mut row = Vec::new();
                        for column in columns.clone() {
                            if let Object::Column{title: _, data} = column {
                                row.push(data[i].user_print())
                            } else {
                                panic!("Expected Column")
                            }
                        }
                        table.add_row(Row::new(row))
                    }
                }
                table.render()
            }
        }
    }
}

/// A utility struct to work around inability to `impl Display for Vec<Object>`
pub struct Objects(Vec<Object>);