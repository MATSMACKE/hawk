use crate::Interpreter;

// Common types used throughout the interpreter
use hawk_common::object::Object;

use rust_decimal::{Decimal, MathematicalOps};

#[allow(unused_imports)]
use rust_decimal_macros::dec;

impl Interpreter {
    /// Adds two numbers or strings
    pub fn add(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Ok(Object::Int(x + y)),
                Object::Decimal(y) => Ok(Object::Decimal(Decimal::from(x as i64) + y)),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Ok(Object::Uncertain {
                    value: Decimal::from(x as i64) + y,
                    uncertainty: u,
                }),
                _ => Err((format!("Can't add Int to {}", operand2.user_print(line)?), line)),
            },
            Object::Decimal(x) => match operand2 {
                Object::Int(y) => Ok(Object::Decimal(x + Decimal::from(y as i64))),
                Object::Decimal(y) => Ok(Object::Decimal(x + y)),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Ok(Object::Uncertain {
                    value: x + y,
                    uncertainty: u,
                }),
                _ => Err((format!("Can't add Decimal to {}", operand2.user_print(line)?), line)),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Ok(Object::Uncertain {
                    value: x + (Decimal::from(y as i64)),
                    uncertainty: u1,
                }),
                Object::Decimal(y) => Ok(Object::Uncertain {
                    value: x + y,
                    uncertainty: u1,
                }),
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Ok(Object::Uncertain {
                    value: x + y,
                    uncertainty: u1 + u2,
                }),
                _ => Err((format!("Can't add Uncertain to {}", operand2.user_print(line)?), line)),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::add(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        )?)
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::add(operand1, operand2.clone(), line)?)
                    }
                }
                Ok(Object::Column(results))
            }
            Object::String(x) => match operand2 {
                Object::String(y) => Ok(Object::String(format!("{x}{y}"))),
                _ => Err((format!("Can't add String to {}", operand2.user_print(line)?), line)),
            },
            _ => Err((format!("Can't add {}", operand1.user_print(line)?), line)),
        }
    }

    /// Subtracts two numbers
    pub fn subtract(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Ok(Object::Int(x - y)),
                Object::Decimal(y) => Ok(Object::Decimal(Decimal::from(x as i64) - y)),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Ok(Object::Uncertain {
                    value: (Decimal::from(x as i64)) - y,
                    uncertainty: u,
                }),
                _ => Err((format!("Can't subtract {} from Int", operand2.user_print(line)?), line)),
            },
            Object::Decimal(x) => match operand2 {
                Object::Int(y) => Ok(Object::Decimal(x - (Decimal::from(y as i64)))),
                Object::Decimal(y) => Ok(Object::Decimal(x - y)),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Ok(Object::Uncertain {
                    value: x - y,
                    uncertainty: u,
                }),
                _ => Err((format!("Can't subtract {} from Decimal", operand2.user_print(line)?), line)),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Ok(Object::Uncertain {
                    value: x - (Decimal::from(y as i64)),
                    uncertainty: u1,
                }),
                Object::Decimal(y) => Ok(Object::Uncertain {
                    value: x - y,
                    uncertainty: u1,
                }),
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Ok(Object::Uncertain {
                    value: x - y,
                    uncertainty: u1 + u2,
                }),
                _ => Err((format!("Can't subtract {} from Uncertain", operand2.user_print(line)?), line)),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, minuend) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::subtract(
                            minuend.clone(),
                            operand2_data[index].clone(),
                            line,
                        )?)
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::subtract(operand1, operand2.clone(), line)?)
                    }
                }
                Ok(Object::Column(results))
            }
            _ => Err((format!("Can't subtract {}", operand1.user_print(line)?), line)),
        }
    }

    /// Multiplies two numbers
    pub fn multiply(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Ok(Object::Int(x * y)),
                Object::Decimal(y) => Ok(Object::Decimal(Decimal::from(x as i64) * y)),
                Object::Uncertain { value, uncertainty } => Ok(Object::Uncertain {
                    value: value * (Decimal::from(x as i64)),
                    uncertainty: uncertainty * (Decimal::from(x as i64)),
                }),
                _ => Err((format!("Can't multiply Int by {}", operand2.user_print(line)?), line)),
            },
            Object::Decimal(x) => match operand2 {
                Object::Int(y) => Ok(Object::Decimal(x * (Decimal::from(y as i64)))),
                Object::Decimal(y) => Ok(Object::Decimal(x * y)),
                Object::Uncertain { value, uncertainty } => Ok(Object::Uncertain {
                    value: value * x,
                    uncertainty: uncertainty * x,
                }),
                _ => Err((format!("Can't multiply Decimal by {}", operand2.user_print(line)?), line)),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Ok(Object::Uncertain {
                    value: x * (Decimal::from(y as i64)),
                    uncertainty: u1 * (Decimal::from(y as i64)),
                }),
                Object::Decimal(y) => Ok(Object::Uncertain {
                    value: x * y,
                    uncertainty: u1 * y,
                }),
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Ok(Object::Uncertain {
                    value: x * y,
                    uncertainty: x * y * ((u1 / x) + (u2 / y)),
                }),
                _ => Err((format!("Can't multiply Uncertain by {}", operand2.user_print(line)?), line)),
            },
            Object::Column(operand1_data) => {
                let mut products: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        products.push(Interpreter::multiply(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        )?)
                    }
                } else {
                    for operand1 in operand1_data {
                        products.push(Interpreter::multiply(operand1, operand2.clone(), line)?)
                    }
                }
                Ok(Object::Column(products))
            }
            _ => Err((format!("Can't multiply {}", operand1.user_print(line)?), line)),
        }
    }

    /// Divides two numbers
    pub fn divide(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Ok(Object::Int(x / y)),
                Object::Decimal(y) => Ok(Object::Decimal(Decimal::from(x as i64) / y)),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Ok(Object::Uncertain {
                    value: (Decimal::from(x as i64)) / y,
                    uncertainty: (Decimal::from(x as i64)) * u / (y * y),
                }),
                _ => Err((format!("Can't divide Int by {}", operand2.user_print(line)?), line)),
            },
            Object::Decimal(x) => match operand2 {
                Object::Int(y) => Ok(Object::Decimal(x / (Decimal::from(y as i64)))),
                Object::Decimal(y) => Ok(Object::Decimal(x / y)),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Ok(Object::Uncertain {
                    value: x / y,
                    uncertainty: x * u / (y * y),
                }),
                _ => Err((format!("Can't divide Decimal by {}", operand2.user_print(line)?), line)),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Ok(Object::Uncertain {
                    value: x / (Decimal::from(y as i64)),
                    uncertainty: u1 / (Decimal::from(y as i64)),
                }),
                Object::Decimal(y) => Ok(Object::Uncertain {
                    value: x / y,
                    uncertainty: u1 / y,
                }),
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Ok(Object::Uncertain {
                    value: x / y,
                    uncertainty: (x / y) * ((u1 / x) + (u2 / y)),
                }),
                _ => Err((format!("Can't divide Uncertain by {}", operand2.user_print(line)?), line)),
            },
            Object::Column(operand1_data) => {
                let mut quotients: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        quotients.push(Interpreter::divide(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        )?)
                    }
                } else {
                    for operand1 in operand1_data {
                        quotients.push(Interpreter::divide(operand1, operand2.clone(), line)?)
                    }
                }
                Ok(Object::Column(quotients))
            }
            _ => Err((format!("Can't divide {}", operand1.user_print(line)?), line)),
        }
    }

    /// Raises a number to the power of another
    pub fn exponent(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Ok(Object::Int(x.pow(y as u32))),
                Object::Decimal(y) => Ok(Object::Decimal((Decimal::from(x as i64)).powd(y))),
                _ => Err((format!("Can't raise Int to {}", operand2.user_print(line)?), line)),
            },
            Object::Decimal(x) => match operand2 {
                Object::Int(y) => Ok(Object::Decimal(x.powi(y as i64))),
                Object::Decimal(y) => Ok(Object::Decimal(x.powd(y))),
                _ => Err((format!("Can't raise Decimal to {}", operand2.user_print(line)?), line)),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Ok(Object::Uncertain {
                    value: x.powd(Decimal::from(y as i64)),
                    uncertainty: x.powd(Decimal::from(y as i64)) * (Decimal::from(y as i64)) * (u1 / x),
                }),
                Object::Decimal(y) => Ok(Object::Uncertain {
                    value: x.powd(y),uncertainty: x.powd(y) * y * (u1 / x),
                }),
                _ => Err((format!("Can't raise Uncertain to {}", operand2.user_print(line)?), line)),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::exponent(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        )?)
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::exponent(operand1, operand2.clone(), line)?)
                    }
                }
                Ok(Object::Column(results))
            }
            _ => Err((format!("Can't exponentiate {}", operand1.user_print(line)?), line)),
        }
    }



    

    /// Checks if object is greater than or equal to another object
    pub fn greaterthanequal(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x >= y))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean((Decimal::from(x as i64)) >= y))
            } else {
                Err((format!("Can't compare Int to {}", operand2.user_print(line)?), line))
            }
        } else if let Object::Decimal(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x >= Decimal::from(y as i64)))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean(x >= y))
            } else {
                Err((format!("Can't compare Decimal to {}", operand2.user_print(line)?), line))
            }
        } else {
            Err((format!("Can't compare {}", operand1.user_print(line)?), line))
        }
    }

    /// Checks if object is greater than another object
    pub fn greaterthan(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x > y))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean((Decimal::from(x as i64)) > y))
            } else {
                Err((format!("Can't compare Int to {}", operand2.user_print(line)?), line))
            }
        } else if let Object::Decimal(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x > Decimal::from(y as i64)))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean(x > y))
            } else {
                Err((format!("Can't compare Decimal to {}", operand2.user_print(line)?), line))
            }
        } else {
            Err((format!("Can't compare {}", operand1.user_print(line)?), line))
        }
    }

    /// Checks if object is less than or equal to another object
    pub fn lessthanequal(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x <= y))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean((Decimal::from(x as i64)) <= y))
            } else {
                Err((format!("Can't compare Int to {}", operand2.user_print(line)?), line))
            }
        } else if let Object::Decimal(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x <= Decimal::from(y as i64)))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean(x <= y))
            } else {
                Err((format!("Can't compare Decimal to {}", operand2.user_print(line)?), line))
            }
        } else {
            Err((format!("Can't compare {}", operand1.user_print(line)?), line))
        }
    }

    /// Checks if object is less than another object
    pub fn lessthan(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x < y))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean((Decimal::from(x as i64)) < y))
            } else {
                Err((format!("Can't compare Int to {}", operand2.user_print(line)?), line))
            }
        } else if let Object::Decimal(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x < Decimal::from(y as i64)))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean(x < y))
            } else {
                Err((format!("Can't compare Decimal to {}", operand2.user_print(line)?), line))
            }
        } else {
            Err((format!("Can't compare {}", operand1.user_print(line)?), line))
        }
    }

    /// Checks if object is not equal to another object
    pub fn notequal(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x != y))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean((Decimal::from(x as i64)) != y))
            } else {
                Err((format!("Can't compare Int to {}", operand2.user_print(line)?), line))
            }
        } else if let Object::Decimal(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x != Decimal::from(y as i64)))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean(x != y))
            } else {
                Err((format!("Can't compare Decimal to {}", operand2.user_print(line)?), line))
            }
        } else {
            Err((format!("Can't compare {}", operand1.user_print(line)?), line))
        }
    }

    /// Checks if object is equal to another object
    pub fn equalequal(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x == y))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean((Decimal::from(x as i64)) == y))
            } else {
                Err((format!("Can't compare Int to {}", operand2.user_print(line)?), line))
            }
        } else if let Object::Decimal(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Ok(Object::Boolean(x == Decimal::from(y as i64)))
            } else if let Object::Decimal(y) = operand2 {
                Ok(Object::Boolean(x == y))
            } else {
                Err((format!("Can't compare Decimal to {}", operand2.user_print(line)?), line))
            }
        } else {
            Err((format!("Can't compare {}", operand1.user_print(line)?), line))
        }
    }

    /// Performs logical AND on two booleans
    pub fn and(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Boolean(op1) = operand1 {
            if let Object::Boolean(op2) = operand2 {
                Ok(Object::Boolean(op1 && op2))
            } else {
                Err((
                    format!(
                        "Logical operations can only be performed on booleans, not {}",
                        operand2.user_print(line)?
                    ),
                    line,
                ))
            }
        } else {
            Err((
                format!("Logical operations can only be performed on booleans, not {}", operand1.user_print(line)?),
                line,
            ))
        }
    }

    /// Performs logical OR on two booleans
    pub fn or(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Boolean(op1) = operand1 {
            if let Object::Boolean(op2) = operand2 {
                Ok(Object::Boolean(op1 || op2))
            } else {
                Err((
                    format!(
                        "Logical operations can only be performed on booleans, not {}",
                        operand2.user_print(line)?
                    ),
                    line,
                ))
            }
        } else {
            Err((
                format!("Logical operations can only be performed on booleans, not {}", operand1.user_print(line)?),
                line,
            ))
        }
    }

    /// Performs logical NOT on a boolean
    pub fn not(eval_op: Object, line: usize) -> Result<Object, (String, usize)> {
        if let Object::Boolean(x) = eval_op {
            Ok(Object::Boolean(!x))
        } else {
            Err((
                format!("Logical operations can only be performed on booleans, not {}", eval_op.user_print(line)?),
                line,
            ))
        }
    }

    /// Negates a number
    pub fn negate(eval_op: Object, line: usize) -> Result<Object, (String, usize)> {
        match eval_op {
            Object::Int(x) => Ok(Object::Int(-x)),
            Object::Decimal(x) => Ok(Object::Decimal(-x)),
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                for operand1 in operand1_data {
                    results.push(Interpreter::negate(operand1, line)?)
                }
                Ok(Object::Column(results))
            }
            _ => Err((format!("Expected number, found {}", eval_op.user_print(line)?), line)),
        }
    }

    /// Adds an uncertainty to a number
    pub fn make_uncertain(operand1: Object, operand2: Object, line: usize) -> Result<Object, (String, usize)> {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Ok(Object::Uncertain {
                    value: Decimal::from(x as i64),
                    uncertainty: Decimal::from(y as i64),
                }),
                Object::Decimal(y) => Ok(Object::Uncertain {
                    value: Decimal::from(x as i64),
                    uncertainty: y,
                }),
                _ => Err((format!("Can't add {} as uncertainty", operand2.user_print(line)?), line)),
            },
            Object::Decimal(x) => match operand2 {
                Object::Int(y) => Ok(Object::Uncertain {
                    value: x,
                    uncertainty: Decimal::from(y as i64),
                }),
                Object::Decimal(y) => Ok(Object::Uncertain {
                    value: x,
                    uncertainty: y,
                }),
                _ => Err((format!("Can't add {} as uncertainty", operand2.user_print(line)?), line)),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::make_uncertain(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        )?)
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::make_uncertain(operand1, operand2.clone(), line)?)
                    }
                }
                Ok(Object::Column(results))
            }
            _ => Err((format!("Can't add uncertainty to {}", operand1.user_print(line)?), line)),
        }
    }
}

#[test]
fn addition() {
    assert_eq!(Interpreter::add(Object::Int(4), Object::Int(5), 0), Ok(Object::Int(9)));
    assert_eq!(Interpreter::add(Object::Int(4), Object::Decimal(dec!(5.1)), 0), Ok(Object::Decimal(dec!(9.1))));
    assert_eq!(Interpreter::add(Object::Decimal(dec!(4.2)), Object::Int(5), 0), Ok(Object::Decimal(dec!(9.2))));
    assert_eq!(Interpreter::add(Object::Decimal(dec!(4.3)), Object::Decimal(dec!(5.2)), 0),Ok(Object::Decimal(dec!(9.5))));
    if let Ok(Object::Uncertain { value, uncertainty }) = Interpreter::add(
        Object::Uncertain {
            value: dec!(1.0),
            uncertainty: dec!(0.1),
        },
        Object::Int(3),
        1,
    ) {
        assert_eq!(value, dec!(4.0));
        assert_eq!(uncertainty, dec!(0.1))
    }
    assert_eq!(
        Interpreter::add(
            Object::String(String::from("Hello ")),
            Object::String(String::from("World")),
            0
        ),
        Ok(Object::String(String::from("Hello World")))
    );
}

#[test]
fn multiply() {
    assert_eq!(Interpreter::multiply(Object::Int(4), Object::Int(5), 0), Ok(Object::Int(20)));
    assert_eq!(Interpreter::multiply(Object::Int(4), Object::Decimal(dec!(5.1)), 0), Ok(Object::Decimal(dec!(20.4))));
    assert_eq!(Interpreter::multiply(Object::Decimal(dec!(1.5)), Object::Int(2), 0), Ok(Object::Decimal(dec!(3.0))));
    assert_eq!(
        Interpreter::multiply(Object::Decimal(dec!(1.5)), Object::Decimal(dec!(1.5)), 0),
        Ok(Object::Decimal(dec!(2.25)))
    );
    if let Ok(Object::Uncertain { value, uncertainty }) = Interpreter::multiply(
        Object::Uncertain {
            value: dec!(6.8),
            uncertainty: dec!(0.2),
        },
        Object::Uncertain {
            value: dec!(3.75),
            uncertainty: dec!(0.05),
        },
        1,
    ) {
        assert_eq!(value, dec!(25.5));
        assert_eq!(uncertainty, dec!(1.09))
    }
}
