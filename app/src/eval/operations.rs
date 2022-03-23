use crate::error::exit;

use crate::eval::Interpreter;

// Common types used throughout the interpreter
use hawk_common::object::Object;

impl Interpreter {
    /// Adds two numbers or strings
    pub fn add(operand1: Object, operand2: Object, line: usize) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Int(x + y),
                Object::Float(y) => Object::Float(x as f64 + y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: (x as f64) + y,
                    uncertainty: u,
                },
                _ => exit(&format!("Can't add Int to {}", operand2), line),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Float(x + (y as f64)),
                Object::Float(y) => Object::Float(x + y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: x + y,
                    uncertainty: u,
                },
                _ => exit(&format!("Can't add Float to {}", operand2), line),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Object::Uncertain {
                    value: x + (y as f64),
                    uncertainty: u1,
                },
                Object::Float(y) => Object::Uncertain {
                    value: x + y,
                    uncertainty: u1,
                },
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Object::Uncertain {
                    value: x + y,
                    uncertainty: u1 + u2,
                },
                _ => exit(&format!("Can't add Uncertain to {}", operand2), line),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::add(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        ))
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::add(operand1, operand2.clone(), line))
                    }
                }
                Object::Column(results)
            }
            Object::String(x) => match operand2 {
                Object::String(y) => Object::String(format!("{x}{y}")),
                _ => exit(&format!("Can't add String to {}", operand2), line),
            },
            _ => exit(&format!("Can't add {}", operand1), line),
        }
    }

    /// Subtracts two numbers
    pub fn subtract(operand1: Object, operand2: Object, line: usize) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Int(x - y),
                Object::Float(y) => Object::Float(x as f64 - y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: (x as f64) - y,
                    uncertainty: u,
                },
                _ => exit(&format!("Can't subtract {} from Int", operand2), line),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Float(x - (y as f64)),
                Object::Float(y) => Object::Float(x - y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: x - y,
                    uncertainty: u,
                },
                _ => exit(&format!("Can't subtract {} from Float", operand2), line),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Object::Uncertain {
                    value: x - (y as f64),
                    uncertainty: u1,
                },
                Object::Float(y) => Object::Uncertain {
                    value: x - y,
                    uncertainty: u1,
                },
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Object::Uncertain {
                    value: x - y,
                    uncertainty: u1 + u2,
                },
                _ => exit(&format!("Can't subtract {} from Uncertain", operand2), line),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, minuend) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::subtract(
                            minuend.clone(),
                            operand2_data[index].clone(),
                            line,
                        ))
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::subtract(operand1, operand2.clone(), line))
                    }
                }
                Object::Column(results)
            }
            _ => exit(&format!("Can't subtract {}", operand1), line),
        }
    }

    /// Multiplies two numbers
    pub fn multiply(operand1: Object, operand2: Object, line: usize) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Int(x * y),
                Object::Float(y) => Object::Float(x as f64 * y),
                Object::Uncertain { value, uncertainty } => Object::Uncertain {
                    value: value * (x as f64),
                    uncertainty: uncertainty * (x as f64),
                },
                _ => exit(&format!("Can't multiply Int by {}", operand2), line),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Float(x * (y as f64)),
                Object::Float(y) => Object::Float(x * y),
                Object::Uncertain { value, uncertainty } => Object::Uncertain {
                    value: value * x,
                    uncertainty: uncertainty * x,
                },
                _ => exit(&format!("Can't multiply Float by {}", operand2), line),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Object::Uncertain {
                    value: x * (y as f64),
                    uncertainty: u1 * (y as f64),
                },
                Object::Float(y) => Object::Uncertain {
                    value: x * y,
                    uncertainty: u1 * y,
                },
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Object::Uncertain {
                    value: x * y,
                    uncertainty: x * y * ((u1 / x) + (u2 / y)),
                },
                _ => exit(&format!("Can't multiply Uncertain by {}", operand2), line),
            },
            Object::Column(operand1_data) => {
                let mut products: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        products.push(Interpreter::multiply(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        ))
                    }
                } else {
                    for operand1 in operand1_data {
                        products.push(Interpreter::multiply(operand1, operand2.clone(), line))
                    }
                }
                Object::Column(products)
            }
            _ => exit(&format!("Can't multiply {}", operand1), line),
        }
    }

    /// Divides two numbers
    pub fn divide(operand1: Object, operand2: Object, line: usize) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Int(x / y),
                Object::Float(y) => Object::Float(x as f64 / y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: (x as f64) / y,
                    uncertainty: (x as f64) * u / (y * y),
                },
                _ => exit(&format!("Can't divide Int by {}", operand2), line),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Float(x / (y as f64)),
                Object::Float(y) => Object::Float(x / y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: x / y,
                    uncertainty: x * u / (y * y),
                },
                _ => exit(&format!("Can't divide Float by {}", operand2), line),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Object::Uncertain {
                    value: x / (y as f64),
                    uncertainty: u1 / (y as f64),
                },
                Object::Float(y) => Object::Uncertain {
                    value: x / y,
                    uncertainty: u1 / y,
                },
                Object::Uncertain {
                    value: y,
                    uncertainty: u2,
                } => Object::Uncertain {
                    value: x / y,
                    uncertainty: (x / y) * ((u1 / x) + (u2 / y)),
                },
                _ => exit(&format!("Can't divide Uncertain by {}", operand2), line),
            },
            Object::Column(operand1_data) => {
                let mut quotients: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        quotients.push(Interpreter::divide(
                            operand1.clone(),
                            operand2_data[index].clone(),
                            line,
                        ))
                    }
                } else {
                    for operand1 in operand1_data {
                        quotients.push(Interpreter::add(operand1, operand2.clone(), line))
                    }
                }
                Object::Column(quotients)
            }
            _ => exit(&format!("Can't divide {}", operand1), line),
        }
    }

    /// Raises a number to the power of another
    pub fn exponent(operand1: Object, operand2: Object, line: usize) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Int(x.pow(y as u32)),
                Object::Float(y) => Object::Float((x as f64).powf(y)),
                _ => exit(&format!("Can't raise Int to {}", operand2), line),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Float(x.powf(y as f64)),
                Object::Float(y) => Object::Float(x.powf(y)),
                _ => exit(&format!("Can't raise Float to {}", operand2), line),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match operand2 {
                Object::Int(y) => Object::Uncertain {
                    value: x.powf(y as f64),
                    uncertainty: x.powf(y as f64) * (y as f64) * (u1 / x),
                },
                Object::Float(y) => Object::Uncertain {
                    value: x.powf(y),
                    uncertainty: x.powf(y) * y * (u1 / x),
                },
                _ => exit(&format!("Can't raise Uncertain to {}", operand2), line),
            },
            _ => exit(&format!("Can't exponentiate {}", operand1), line),
        }
    }

    /// Checks if object is greater than or equal to another object
    pub fn greaterthanequal(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x >= y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) >= y)
            } else {
                exit(&format!("Can't compare Int to {}", operand2), line)
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x >= y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x >= y)
            } else {
                exit(&format!("Can't compare Float to {}", operand2), line)
            }
        } else {
            exit(&format!("Can't compare {}", operand1), line)
        }
    }

    /// Checks if object is greater than another object
    pub fn greaterthan(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x > y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) > y)
            } else {
                exit(&format!("Can't compare Int to {}", operand2), line)
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x > y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x > y)
            } else {
                exit(&format!("Can't compare Float to {}", operand2), line)
            }
        } else {
            exit(&format!("Can't compare {}", operand1), line)
        }
    }

    /// Checks if object is less than or equal to another object
    pub fn lessthanequal(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x <= y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) <= y)
            } else {
                exit(&format!("Can't compare Int to {}", operand2), line)
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x <= y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x <= y)
            } else {
                exit(&format!("Can't compare Float to {}", operand2), line)
            }
        } else {
            exit(&format!("Can't compare {}", operand1), line)
        }
    }

    /// Checks if object is less than another object
    pub fn lessthan(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x < y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) < y)
            } else {
                exit(&format!("Can't compare Int to {}", operand2), line)
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x < y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x < y)
            } else {
                exit(&format!("Can't compare Float to {}", operand2), line)
            }
        } else {
            exit(&format!("Can't compare {}", operand1), line)
        }
    }

    /// Checks if object is not equal to another object
    pub fn notequal(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x != y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) != y)
            } else {
                exit(&format!("Can't compare Int to {}", operand2), line)
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x != y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x != y)
            } else {
                exit(&format!("Can't compare Float to {}", operand2), line)
            }
        } else {
            exit(&format!("Can't compare {}", operand1), line)
        }
    }

    /// Checks if object is equal to another object
    pub fn equalequal(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x == y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) == y)
            } else {
                exit(&format!("Can't compare Int to {}", operand2), line)
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x == y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x == y)
            } else {
                exit(&format!("Can't compare Float to {}", operand2), line)
            }
        } else {
            exit(&format!("Can't compare {}", operand1), line)
        }
    }

    /// Performs logical AND on two booleans
    pub fn and(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Boolean(op1) = operand1 {
            if let Object::Boolean(op2) = operand2 {
                Object::Boolean(op1 && op2)
            } else {
                exit(
                    &format!(
                        "Logical operations can only be performed on booleans, not {}",
                        operand2
                    ),
                    line,
                )
            }
        } else {
            exit(
                &format!("Logical operations can only be performed on booleans, not {}", operand1),
                line,
            )
        }
    }

    /// Performs logical OR on two booleans
    pub fn or(operand1: Object, operand2: Object, line: usize) -> Object {
        if let Object::Boolean(op1) = operand1 {
            if let Object::Boolean(op2) = operand2 {
                Object::Boolean(op1 || op2)
            } else {
                exit(
                    &format!(
                        "Logical operations can only be performed on booleans, not {}",
                        operand2
                    ),
                    line,
                )
            }
        } else {
            exit(
                &format!("Logical operations can only be performed on booleans, not {}", operand1),
                line,
            )
        }
    }

    /// Performs logical NOT on a boolean
    pub fn not(eval_op: Object, line: usize) -> Object {
        if let Object::Boolean(x) = eval_op {
            Object::Boolean(!x)
        } else {
            exit(
                &format!("Logical operations can only be performed on booleans, not {}", eval_op),
                line,
            )
        }
    }

    /// Negates a number
    pub fn negate(eval_op: Object, line: usize) -> Object {
        if let Object::Int(x) = eval_op {
            Object::Int(-x)
        } else if let Object::Float(x) = eval_op {
            Object::Float(-x)
        } else {
            exit(&format!("Expected number, found {}", eval_op), line)
        }
    }

    /// Adds an uncertainty to a number
    pub fn make_uncertain(operand1: Object, operand2: Object, line: usize) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Uncertain {
                    value: x as f64,
                    uncertainty: y as f64,
                },
                Object::Float(y) => Object::Uncertain {
                    value: x as f64,
                    uncertainty: y,
                },
                _ => exit(&format!("Can't add {} as uncertainty", operand2), line),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Uncertain {
                    value: x,
                    uncertainty: y as f64,
                },
                Object::Float(y) => Object::Uncertain {
                    value: x,
                    uncertainty: y,
                },
                _ => exit(&format!("Can't add {} as uncertainty", operand2), line),
            },
            _ => exit(&format!("Can't add uncertainty to {}", operand1), line),
        }
    }
}

#[test]
fn addition() {
    use float_cmp::approx_eq;

    assert_eq!(Interpreter::add(Object::Int(4), Object::Int(5), 0), Object::Int(9));
    assert_eq!(Interpreter::add(Object::Int(4), Object::Float(5.1), 0), Object::Float(9.1));
    assert_eq!(Interpreter::add(Object::Float(4.2), Object::Int(5), 0), Object::Float(9.2));
    assert_eq!(Interpreter::add(Object::Float(4.3), Object::Float(5.2), 0), Object::Float(9.5));
    if let Object::Uncertain{value, uncertainty} = Interpreter::add(Object::Uncertain{value: 1.0, uncertainty: 0.1},  Object::Int(3), 1) {
        assert!(approx_eq!(f64, value, 4.0, ulps = 3));
        assert!(approx_eq!(f64, uncertainty, 0.1, ulps = 3))
    }
    assert_eq!(Interpreter::add(Object::String(String::from("Hello ")), Object::String(String::from("World")), 0), Object::String(String::from("Hello World")));
}

#[test]
fn multiply() {
    use float_cmp::approx_eq;

    assert_eq!(Interpreter::multiply(Object::Int(4), Object::Int(5), 0), Object::Int(20));
    assert_eq!(Interpreter::multiply(Object::Int(4), Object::Float(5.1), 0), Object::Float(20.4));
    assert_eq!(Interpreter::multiply(Object::Float(1.5), Object::Int(2), 0), Object::Float(3.0));
    assert_eq!(Interpreter::multiply(Object::Float(1.5), Object::Float(1.5), 0), Object::Float(2.25));
    if let Object::Uncertain{value, uncertainty} = Interpreter::multiply(Object::Uncertain{value: 6.8, uncertainty: 0.2},  Object::Uncertain{value: 3.75, uncertainty: 0.05}, 1) {
        assert!(approx_eq!(f64, value, 25.5, ulps = 3));
        assert!(approx_eq!(f64, uncertainty, 1.09, ulps = 3))
    }
}
