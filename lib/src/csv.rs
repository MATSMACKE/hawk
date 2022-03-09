use crate::object::Object;
use crate::token::{Token, TokenType};
use core::panic;
use std::fs::write;

/*pub fn csv_to_datatable(filename: String) -> Object {

}*/

/// Writes an `Object::DataTable` to a `.csv` file
pub fn datatable_to_csv(filename: String, datatable: Object) {
    if let Object::DataTable(_) = &datatable {
        let str = datatable.format_for_csv();
        if let Ok(()) = write(&filename, str) {
            ()
        } else {
            panic!("Couldn't write to file: {filename}")
        }
    } else {
        panic!("Expected DataTable, instead got {}", datatable.user_print())
    }
}

impl Object {
    /// Generates a string representation of the Object that is suitable for a `.csv` file
    pub fn format_for_csv(&self) -> String {
        match self.clone() {
            Self::Boolean(x) =>format!("{x}"),
            Self::Float(x) => format!("{x}"),
            Self::Int(x) => format!("{x}"),
            Self::String(x) => format!("{x}"),
            Self::Uncertain{value, uncertainty: _} => format!("{value}"),
            Self::DataTable(columns) => {
                let mut str = String::from("");
                for (idx, col) in columns.iter().enumerate() {
                    if let Object::Column{title, data: _} = col {
                        if idx < columns.len() - 1 {
                            str = format!("{str}{title}, ");
                        } else {
                            str = format!("{str}{title}");
                        }
                    } else {
                        panic!("Expected column, instead found {}", col.user_print())
                    }
                    
                }
                let len: usize;
                if let Object::Column{title: _, data} = columns[0].clone() {
                    len = data.len()
                } else {
                    panic!("Expected Column, instead got {}", columns[0].user_print())
                }
                for i in 0..len {
                    str = format!("{str}\n");
                    for (idx, column) in columns.iter().enumerate() {
                        if let Object::Column{title: _, data} = column {
                            if idx < columns.len() - 1 {
                                str = format!("{str}{}, ", data[i].format_for_csv());
                            } else {
                                str = format!("{str}{}", data[i].format_for_csv());
                            }
                        }
                    }
                }
                str
            },
            _ => panic!("Can't write {self} to csv")
        }
    }
}