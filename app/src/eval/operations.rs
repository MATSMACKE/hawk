use crate::eval::Interpreter;

// Common types used throughout the interpreter
use crate::object::Object;

impl Interpreter {
    /// Adds two numbers or strings
    pub fn add(operand1: Object, operand2: Object) -> Object {
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
                _ => panic!("Can't add Int to {}", operand2),
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
                _ => panic!("Can't add Float to {}", operand2),
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
                _ => panic!("Can't add Uncertain to {}", operand2),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::add(operand1.clone(), operand2_data[index].clone()))
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::add(operand1, operand2.clone()))
                    }
                }
                Object::Column(results)
            }
            Object::String(x) => match operand2 {
                Object::String(y) => Object::String(format!("{x}{y}")),
                _ => panic!("Can't add String to {}", operand2),
            },
            _ => panic!("Can't add {operand1}"),
        }
    }

    /// Subtracts two numbers
    pub fn subtract(operand1: Object, operand2: Object) -> Object {
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
                _ => panic!("Can't subtract {} from Int", operand2),
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
                _ => panic!("Can't subtract {} from Float", operand2),
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
                _ => panic!("Can't subtract {} from Uncertain", operand2),
            },
            Object::Column(operand1_data) => {
                let mut results: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, minuend) in operand1_data.iter().enumerate() {
                        results.push(Interpreter::subtract(
                            minuend.clone(),
                            operand2_data[index].clone(),
                        ))
                    }
                } else {
                    for operand1 in operand1_data {
                        results.push(Interpreter::subtract(operand1, operand2.clone()))
                    }
                }
                Object::Column(results)
            }
            _ => panic!("Can't subtract {operand1}"),
        }
    }

    /// Multiplies two numbers
    pub fn multiply(operand1: Object, operand2: Object) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Int(x * y),
                Object::Float(y) => Object::Float(x as f64 * y),
                Object::Uncertain { value, uncertainty } => Object::Uncertain {
                    value: value * (x as f64),
                    uncertainty: uncertainty * (x as f64),
                },
                _ => panic!("Can't multiply Int by {}", operand2),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Float(x * (y as f64)),
                Object::Float(y) => Object::Float(x * y),
                Object::Uncertain { value, uncertainty } => Object::Uncertain {
                    value: value * x,
                    uncertainty: uncertainty * x,
                },
                _ => panic!("Can't multiply Float by {}", operand2),
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
                _ => panic!("Can't multiply Uncertain by {}", operand2),
            },
            Object::Column(operand1_data) => {
                let mut products: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        products.push(Interpreter::multiply(
                            operand1.clone(),
                            operand2_data[index].clone(),
                        ))
                    }
                } else {
                    for operand1 in operand1_data {
                        products.push(Interpreter::multiply(operand1, operand2.clone()))
                    }
                }
                Object::Column(products)
            }
            _ => panic!("Can't multiply {operand1}"),
        }
    }

    /// Divides two numbers
    pub fn divide(operand1: Object, operand2: Object) -> Object {
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
                _ => panic!("Can't divide Int by {}", operand2),
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
                _ => panic!("Can't divide Float by {}", operand2),
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
                _ => panic!("Can't divide Uncertain by {}", operand2),
            },
            Object::Column(operand1_data) => {
                let mut quotients: Vec<Object> = Vec::new();
                if let Object::Column(operand2_data) = operand2 {
                    for (index, operand1) in operand1_data.iter().enumerate() {
                        quotients.push(Interpreter::divide(
                            operand1.clone(),
                            operand2_data[index].clone(),
                        ))
                    }
                } else {
                    for operand1 in operand1_data {
                        quotients.push(Interpreter::add(operand1, operand2.clone()))
                    }
                }
                Object::Column(quotients)
            }
            _ => panic!("Can't divide {operand1}"),
        }
    }

    /// Raises a number to the power of another
    pub fn exponent(operand1: Object, operand2: Object) -> Object {
        match operand1 {
            Object::Int(x) => match operand2 {
                Object::Int(y) => Object::Int(x.pow(y as u32)),
                Object::Float(y) => Object::Float((x as f64).powf(y)),
                _ => panic!("Can't raise Int to {}", operand2),
            },
            Object::Float(x) => match operand2 {
                Object::Int(y) => Object::Float(x.powf(y as f64)),
                Object::Float(y) => Object::Float(x.powf(y)),
                _ => panic!("Can't raise Float to {}", operand2),
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
                _ => panic!("Can't raise Uncertain to {}", operand2),
            },
            _ => panic!("Can't exponentiate {operand1}"),
        }
    }

    /// Checks if object is greater than or equal to another object
    pub fn greaterthanequal(operand1: Object, operand2: Object) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x >= y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) >= y)
            } else {
                panic!("Can't compare Int to this type")
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x >= y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x >= y)
            } else {
                panic!("Can't compare Float to this type")
            }
        } else {
            panic!("Can't compare non-numbers")
        }
    }

    /// Checks if object is greater than another object
    pub fn greaterthan(operand1: Object, operand2: Object) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x > y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) > y)
            } else {
                panic!("Can't compare Int to this type")
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x > y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x > y)
            } else {
                panic!("Can't compare Float to this type")
            }
        } else {
            panic!("Can't compare non-numbers")
        }
    }

    /// Checks if object is less than or equal to another object
    pub fn lessthanequal(operand1: Object, operand2: Object) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x <= y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) <= y)
            } else {
                panic!("Can't compare Int to this type")
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x <= y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x <= y)
            } else {
                panic!("Can't compare Float to this type")
            }
        } else {
            panic!("Can't compare non-numbers")
        }
    }

    /// Checks if object is less than another object
    pub fn lessthan(operand1: Object, operand2: Object) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x < y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) < y)
            } else {
                panic!("Can't compare Int to this type")
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x < y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x < y)
            } else {
                panic!("Can't compare Float to this type")
            }
        } else {
            panic!("Can't compare non-numbers")
        }
    }

    /// Checks if object is not equal to another object
    pub fn notequal(operand1: Object, operand2: Object) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x != y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) != y)
            } else {
                panic!("Can't compare Int to this type")
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x != y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x != y)
            } else {
                panic!("Can't compare Float to this type")
            }
        } else {
            panic!("Can't compare non-numbers")
        }
    }

    /// Checks if object is equal to another object
    pub fn equalequal(operand1: Object, operand2: Object) -> Object {
        if let Object::Int(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x == y)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean((x as f64) == y)
            } else {
                panic!("Can't compare Int to this type")
            }
        } else if let Object::Float(x) = operand1 {
            if let Object::Int(y) = operand2 {
                Object::Boolean(x == y as f64)
            } else if let Object::Float(y) = operand2 {
                Object::Boolean(x == y)
            } else {
                panic!("Can't compare Float to this type")
            }
        } else {
            panic!("Can't compare non-numbers")
        }
    }

    /// Performs logical AND on two booleans
    pub fn and(operand1: Object, operand2: Object) -> Object {
        if let Object::Boolean(op1) = operand1 {
            if let Object::Boolean(op2) = operand2 {
                Object::Boolean(op1 && op2)
            } else {
                panic!("Logical operations can only be performed on booleans")
            }
        } else {
            panic!("Logical operations can only be performed on booleans")
        }
    }

    /// Performs logical OR on two booleans
    pub fn or(operand1: Object, operand2: Object) -> Object {
        if let Object::Boolean(op1) = operand1 {
            if let Object::Boolean(op2) = operand2 {
                Object::Boolean(op1 || op2)
            } else {
                panic!("Logical operations can only be performed on booleans")
            }
        } else {
            panic!("Logical operations can only be performed on booleans")
        }
    }

    /// Performs logical NOT on a boolean
    pub fn not(eval_op: Object) -> Object {
        if let Object::Boolean(x) = eval_op {
            Object::Boolean(!x)
        } else {
            panic!("Expected bool, found {:?}", eval_op)
        }
    }

    /// Negates a number
    pub fn negate(eval_op: Object) -> Object {
        if let Object::Int(x) = eval_op {
            Object::Int(-x)
        } else if let Object::Float(x) = eval_op {
            Object::Float(-x)
        } else {
            panic!("Expected number, found {:?}", eval_op)
        }
    }

    /// Adds an uncertainty to a number
    pub fn make_uncertain(operand1: Object, operand2: Object) -> Object {
        match operand1 {
            Object::Int(x) => {
                match operand2 {
                    Object::Int(y) => Object::Uncertain { value: x as f64, uncertainty: y as f64 },
                    Object::Float(y) => Object::Uncertain { value: x as f64, uncertainty: y },
                    _ => panic!("{operand2} can't be an uncertainty")
                }
            },
            Object::Float(x) => {
                match operand2 {
                    Object::Int(y) => Object::Uncertain { value: x, uncertainty: y as f64 },
                    Object::Float(y) => Object::Uncertain { value: x, uncertainty: y },
                    _ => panic!("{operand2} can't be an uncertainty")
                }
            },
            _ => panic!("Can't add an uncertainty to {operand1}")
        }
    }
}