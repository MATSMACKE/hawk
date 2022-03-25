use term_table::row::Row;
use term_table::{Table, TableStyle};

use hawk_common::object::Object;

use crate::error::exit;

pub trait UserPrintObject {
    fn user_print(&self, line: usize) -> String;
    fn user_print_datatable(names: Vec<String>, data: Vec<Object>, line: usize) -> String;
    fn user_print_column(x: Vec<Object>, line: usize) -> String;
    fn user_print_array(x: Vec<Object>, line: usize) -> String;
}

impl UserPrintObject for Object {
    /// Nicely formatted output for displaying objects with `print`
    fn user_print(&self, line: usize) -> String {
        match self.clone() {
            Self::Boolean(x) =>format!("{x}"),
            Self::Float(x) => format!("{x}"),
            Self::Int(x) => format!("{x}"),
            Self::String(x) => format!("{x}"),
            Self::Identifier(x) => format!("{x}"),
            Self::Function{params, block} => format!("Function: params: {:?}, block: {block}", params),
            Self::Array(x) => {
                Self::user_print_array(x, line)
            },
            Self::Null => String::from("Null"),
            Self::Uncertain{value, uncertainty} => format!("{value} ± {uncertainty}"),
            Self::Column(x) => {
                Self::user_print_column(x, line)
            },
            Self::DataTable{names, data} => {
                Self::user_print_datatable(names, data, line)
            },
            Self::Finder(_) => format!("finder function")
        }
    }

    fn user_print_datatable(names: Vec<String>, data: Vec<Object>, line: usize) -> String {
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
                        row.push(objs[i].user_print(line))
                    } else {
                        exit(&format!("Expected column found {}", column), line);
                    }
                }
                table.add_row(Row::new(row))
            }
        }
        table.render()
    }

    fn user_print_column(x: Vec<Object>, line: usize) -> String {
        let mut str = String::from("[");
        for (idx, obj) in x.iter().enumerate() {
            if idx < x.len() - 1 {
                str = format!("{str}{}, ", obj.user_print(line));
            } else {
                str = format!("{str}{}", obj.user_print(line));
            }
        }
        format!("{str}]")
    }

    fn user_print_array(x: Vec<Object>, line: usize) -> String {
        let mut str = String::from("[");
        for (idx, obj) in x.iter().enumerate() {
            if idx < x.len() - 1 {
                str = format!("{str}{}, ", obj.user_print(line));
            } else {
                str = format!("{str}{}", obj.user_print(line));
            }
        }
        format!("{str}]")
    }
}