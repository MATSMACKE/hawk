use crate::csv::{csv_to_datatable, datatable_to_csv};

use crate::Interpreter;
use hawk_common::object::Object;

use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

impl Interpreter {
    pub fn get_std_rs_fn(&mut self, identifier: String, args: Vec<Object>) -> Result<Option<Object>, (String, usize)> {
        match identifier.as_str() {
            "readfile" => {
                if let Object::String(file) = args[0].clone() {
                    if let Ok(str) = (self.filein_fn)(file.clone()) {
                        Ok(Some(Object::String(str)))
                    } else {
                        Err((
                            format!("Expected string as filename, found {}", file),
                            self.line,
                        ))
                    }
                } else {
                    Ok(Some(Object::Null))
                }
            }
            "writefile" => {
                let val = args[1].clone();
                let file = args[0].clone();
                if let Object::String(str) = val {
                    if let Object::String(filename) = file {
                        if let Ok(()) = (self.fileout_fn)(filename.clone(), str) {
                        } else {
                            return Err((format!("Couldn't write file: {}", filename), self.line))
                        }
                    } else {
                        return Err((format!("Expected string as filename, found {}", file), self.line))
                    }
                }
                Ok(Some(Object::Null))
            }
            "read" => {
                let filename = args[0].clone();
                if let Object::String(filename) = filename {
                    Ok(Some(csv_to_datatable(filename, self.line, self.filein_fn)?))
                } else {
                    Err((
                        format!("Expected string as filename, found {}", filename),
                        self.line,
                    ))
                }
            }
            "write" => {
                let val = args[1].clone();
                let file = args[0].clone();
                if let Object::DataTable { names: _, data: _ } = val {
                    if let Object::String(filename) = file {
                        datatable_to_csv(filename, val, self.line, self.fileout_fn)?;
                    } else {
                        return Err((format!("Expected string as filename, found {}", file), self.line))
                    }
                }
                Ok(Some(Object::Null))
            }
            "pi" => Ok(Some(Object::Decimal(Decimal::PI))),
            "ln10" => Ok(Some(Object::Decimal(dec!(2.30258509299)))),
            "ln" => {
                let x;
                if let Object::Decimal(val) = args[0] {
                    x = val
                } else if let Object::Int(val) = args[0] {
                    x = Decimal::from(val as i64)
                } else {
                    return Ok(None);
                }

                Ok(Some(Object::Decimal(ln(x))))
            }
            "e" => Ok(Some(Object::Decimal(Decimal::E))),
            "sin" => {
                let x;
                if let Object::Decimal(val) = args[0] {
                    x = val
                } else if let Object::Int(val) = args[0] {
                    x = Decimal::from(val as i64)
                } else {
                    return Err((
                        format!("Expected number as argument to sin, found {}", args[0]),
                        self.line,
                    ))
                }

                Ok(Some(Object::Decimal(sin(x))))
            },
            "len" => {
                if let Object::Array(data) = args[0].to_owned() {
                    Ok(Some(Object::Int(data.len() as i128)))
                } else {
                    Err((
                        format!("Expected array as argument to len, found {}", args[0]),
                        self.line,
                    ))
                }
            },
            "str" | "string" => {
                if args.len() == 1 {
                    Ok(Some(Object::String(args[0].user_print(self.line)?)))
                } else {
                    Err((
                        format!("Expected exactly 1 input to str, got {:?}", args),
                        self.line,
                    ))
                }
            },
            "isfloat" | "is_float" | "isDecimal" => {
                if args.len() == 1 {
                    if let Object::Decimal(_) = args[0] {
                        Ok(Some(Object::Boolean(true)))
                    } else {
                        Ok(Some(Object::Boolean(false)))
                    }
                } else {
                    Ok(Some(Object::Boolean(false)))
                }
            },
            "isint" | "is_int" | "isInt" => {
                if args.len() == 1 {
                    if let Object::Int(_) = args[0] {
                        Ok(Some(Object::Boolean(true)))
                    } else {
                        Ok(Some(Object::Boolean(false)))
                    }
                } else {
                    Ok(Some(Object::Boolean(false)))
                }
            },
            "isbool" | "is_bool" | "isBool" => {
                if args.len() == 1 {
                    if let Object::Boolean(_) = args[0] {
                        Ok(Some(Object::Boolean(true)))
                    } else {
                        Ok(Some(Object::Boolean(false)))
                    }
                } else {
                    Ok(Some(Object::Boolean(false)))
                }
            },
            "isstring" | "is_str" | "isStr" | "isstr" | "is_string" | "isString" => {
                if args.len() == 1 {
                    if let Object::String(_) = args[0] {
                        Ok(Some(Object::Boolean(true)))
                    } else {
                        Ok(Some(Object::Boolean(false)))
                    }
                } else {
                    Ok(Some(Object::Boolean(false)))
                }
            },
            "isuncertain" | "is_uncertain" | "isUncertain" | "hasuncertainty" | "has_uncertainty" | "hasUncertainty" => {
                if args.len() == 1 {
                    if let Object::Uncertain{value: _, uncertainty: _} = args[0] {
                        Ok(Some(Object::Boolean(true)))
                    } else {
                        Ok(Some(Object::Boolean(false)))
                    }
                } else {
                    Ok(Some(Object::Boolean(false)))
                }
            },
            "isarray" | "is_arr" | "isArr" | "isarr" | "is_array" | "isArray" => {
                if args.len() == 1 {
                    if let Object::Array(_) = args[0] {
                        Ok(Some(Object::Boolean(true)))
                    } else {
                        Ok(Some(Object::Boolean(false)))
                    }
                } else {
                    Ok(Some(Object::Boolean(false)))
                }
            },
            "isnull" | "is_null" | "isNull" => {
                if args.len() == 1 {
                    if let Object::Null = args[0] {
                        Ok(Some(Object::Boolean(true)))
                    } else {
                        Ok(Some(Object::Boolean(false)))
                    }
                } else {
                    Ok(Some(Object::Boolean(false)))
                }
            },
            _ => Ok(None),
        }
    }
}

fn ln(x: Decimal) -> Decimal {
    x.ln()
}

fn sin(x: Decimal) -> Decimal {
    x.sin()
}
