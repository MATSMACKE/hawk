use std::collections::HashMap;

// Common types used throughout the interpreter
use crate::object::Object;
use crate::tree::Statement;

/// Runs parsed code from the list of statements returned by the parser
pub struct Interpreter {
    statements: Vec<Statement>,           // Parsed code to execute
    globals: HashMap<String, Object>,     // Stores variables in the global scope
    scopes: Vec<HashMap<String, Object>>, // A stack variables in local scopes are stored
    loops: usize, // Contains number of nested loops in order to handle `break`
}

impl Interpreter {
    /// Create an `Interpreter` and run code. `global_state` is used to store the state of the REPL.
    pub fn interpret(
        statements: Vec<Statement>,
        global_state: HashMap<String, Object>,
    ) -> HashMap<String, Object> {
        let mut interpreter = Interpreter {
            statements,
            globals: global_state,
            loops: 0,
            scopes: Vec::new(),
        };

        for index in 0..interpreter.statements.len() {
            interpreter.run_statement(interpreter.statements[index].clone())
        }

        interpreter.globals
    }

    /// Executes a given statement
    pub fn run_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Print(expr) => println!("{}", self.eval_expression(expr).user_print()),

            // Variable definition
            Statement::Definition { name, value } => {
                let val = self.eval_expression(value);
                self.insert_top_scope(name, val)
            }
            Statement::While { condition, block } => {
                let is_true = true;
                self.loops += 1;
                let current_loop = self.loops;
                while is_true {
                    if let Object::Boolean(is_true) = self.eval_expression(condition.clone()) {
                        if is_true && (self.loops == current_loop) {
                            self.run_statement(*block.clone())
                        } else {
                            if !(self.loops == current_loop) {
                                break;
                            } else {
                                self.loops -= 1;
                                break;
                            }
                        }
                    } else {
                        panic!("Expected boolean as condition for if statement")
                    }
                }
            }
            Statement::Loop(block) => {
                self.loops += 1;
                let current_loop = self.loops;
                loop {
                    if self.loops == current_loop {
                        self.run_statement(*block.clone())
                    } else {
                        break;
                    }
                }
            }
            Statement::Break => self.loops -= 1,
            Statement::If { condition, block } => {
                if let Object::Boolean(condition) = self.eval_expression(condition) {
                    if condition {
                        self.run_statement(*block)
                    }
                } else {
                    panic!("Expected boolean as condition for if statement")
                }
            }
            Statement::IfElse {
                condition,
                if_block,
                else_block,
            } => {
                let evaled_condition = self.eval_expression(condition);
                if let Object::Boolean(condition) = evaled_condition {
                    // Make sure condition is a boolean
                    if condition {
                        self.run_statement(*if_block) // Run code in if statement
                    } else {
                        self.run_statement(*else_block) // Run code in else statement
                    }
                } else {
                    panic!(
                        "Expected boolean as condition for if statement, instead got {}",
                        evaled_condition
                    )
                }
            }
            Statement::Block(block) => {
                for statement in block {
                    self.run_statement(statement) // Run each statement in block
                }
            }
            Statement::Function {
                identifier,
                params,
                block,
            } => {
                self.insert_top_scope(identifier, Object::Function { params, block });
                // Define function in top scope
            }
            Statement::Return(expr) => {
                let val = self.eval_expression(expr); // Evaluate returned value
                self.insert_top_scope(String::from("return"), val) // Assign to the variable `"return"` in the top scope, which the function call will see and return
            }
            Statement::Import(expr) => {
                if let Object::String(filename) = self.eval_expression(expr) {
                    self.globals = crate::run_script(filename, self.globals.clone());
                } else {
                    panic!("Expected filename to be a string")
                }
            }
            Statement::Expression(expr) => {
                self.eval_expression(expr);
            }
            Statement::Process {
                readfile,
                writefile,
                block,
            } => {
                self.scopes.push(HashMap::new());
                if let Object::String(readfile) = self.eval_expression(readfile) {
                    let datatable = hawk_lib::csv::csv_to_datatable(readfile);
                    if let Object::DataTable { names, data } = datatable.clone() {
                        self.insert_top_scope(String::from("datatable"), datatable);
                        for (index, name) in names.iter().enumerate() {
                            self.insert_top_scope(name.clone(), data[index].clone())
                        }
                    } else {
                        panic!("expected datatable")
                    }
                }
                self.run_statement(*block);
                let mut columns: Vec<String> = Vec::new();
                let mut values: Vec<Object> = Vec::new();
                for (key, value) in &self.scopes[self.scopes.len() - 1] {
                    if let Object::Column(_) = value {
                        columns.push(key.clone());
                        values.push(value.clone());
                    }
                }
                let filename = self.eval_expression(writefile);
                self.run_fn_std(
                    String::from("write"),
                    vec![
                        filename,
                        Object::DataTable {
                            names: columns,
                            data: values,
                        },
                    ],
                );
            }
            _ => {}
        }
    }

    pub fn get_variable(&self, identifier: String) -> Object {
        let mut index = self.scopes.len() as isize - 1;
        while index >= 0 {
            if let Some(x) = self.scopes[index as usize].get(&identifier) {
                return x.clone();
            }
            index -= 1
        }
        if let Some(x) = self.globals.get(&identifier) {
            x.clone()
        } else {
            Object::Null
        }
    }

    fn insert_top_scope(&mut self, identifier: String, value: Object) {
        if self.scopes.len() > 0 {
            let index = self.scopes.len() - 1;
            self.scopes[index].insert(identifier, value);
        } else {
            self.globals.insert(identifier, value);
        }
    }

    pub fn add(augend: Object, addend: Object) -> Object {
        match augend {
            Object::Int(x) => match addend {
                Object::Int(y) => Object::Int(x + y),
                Object::Float(y) => Object::Float(x as f64 + y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: (x as f64) + y,
                    uncertainty: u,
                },
                _ => panic!("Can't add Int to {}", addend),
            },
            Object::Float(x) => match addend {
                Object::Int(y) => Object::Float(x + (y as f64)),
                Object::Float(y) => Object::Float(x + y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: x + y,
                    uncertainty: u,
                },
                _ => panic!("Can't add Float to {}", addend),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match addend {
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
                _ => panic!("Can't add Uncertain to {}", addend),
            },
            Object::Column(augend_data) => {
                let mut sums: Vec<Object> = Vec::new();
                if let Object::Column(addend_data) = addend {
                    for (index, augend) in augend_data.iter().enumerate() {
                        sums.push(Interpreter::add(augend.clone(), addend_data[index].clone()))
                    }
                } else {
                    for augend in augend_data {
                        sums.push(Interpreter::add(augend, addend.clone()))
                    }
                }
                Object::Column(sums)
            }
            Object::String(x) => match addend {
                Object::String(y) => Object::String(format!("{x}{y}")),
                _ => panic!("Can't add String to {}", addend),
            },
            _ => panic!("Can't add {augend}"),
        }
    }

    pub fn subtract(minuend: Object, subtrahend: Object) -> Object {
        match minuend {
            Object::Int(x) => match subtrahend {
                Object::Int(y) => Object::Int(x - y),
                Object::Float(y) => Object::Float(x as f64 - y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: (x as f64) - y,
                    uncertainty: u,
                },
                _ => panic!("Can't subtract {} from Int", subtrahend),
            },
            Object::Float(x) => match subtrahend {
                Object::Int(y) => Object::Float(x - (y as f64)),
                Object::Float(y) => Object::Float(x - y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: x - y,
                    uncertainty: u,
                },
                _ => panic!("Can't subtract {} from Float", subtrahend),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match subtrahend {
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
                _ => panic!("Can't subtract {} from Uncertain", subtrahend),
            },
            Object::Column(minuend_data) => {
                let mut differences: Vec<Object> = Vec::new();
                if let Object::Column(subtrahend_data) = subtrahend {
                    for (index, minuend) in minuend_data.iter().enumerate() {
                        differences.push(Interpreter::subtract(
                            minuend.clone(),
                            subtrahend_data[index].clone(),
                        ))
                    }
                } else {
                    for minuend in minuend_data {
                        differences.push(Interpreter::subtract(minuend, subtrahend.clone()))
                    }
                }
                Object::Column(differences)
            }
            _ => panic!("Can't subtract {minuend}"),
        }
    }

    pub fn multiply(multiplicand: Object, multiplier: Object) -> Object {
        match multiplicand {
            Object::Int(x) => match multiplier {
                Object::Int(y) => Object::Int(x * y),
                Object::Float(y) => Object::Float(x as f64 * y),
                Object::Uncertain { value, uncertainty } => Object::Uncertain {
                    value: value * (x as f64),
                    uncertainty: uncertainty * (x as f64),
                },
                _ => panic!("Can't multiply Int by {}", multiplier),
            },
            Object::Float(x) => match multiplier {
                Object::Int(y) => Object::Float(x * (y as f64)),
                Object::Float(y) => Object::Float(x * y),
                Object::Uncertain { value, uncertainty } => Object::Uncertain {
                    value: value * x,
                    uncertainty: uncertainty * x,
                },
                _ => panic!("Can't multiply Float by {}", multiplier),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match multiplier {
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
                _ => panic!("Can't multiply Uncertain by {}", multiplier),
            },
            Object::Column(multiplicand_data) => {
                let mut products: Vec<Object> = Vec::new();
                if let Object::Column(multiplier_data) = multiplier {
                    for (index, multiplicand) in multiplicand_data.iter().enumerate() {
                        products.push(Interpreter::multiply(
                            multiplicand.clone(),
                            multiplier_data[index].clone(),
                        ))
                    }
                } else {
                    for multiplicand in multiplicand_data {
                        products.push(Interpreter::multiply(multiplicand, multiplier.clone()))
                    }
                }
                Object::Column(products)
            }
            _ => panic!("Can't multiply {multiplicand}"),
        }
    }

    pub fn divide(dividend: Object, divisor: Object) -> Object {
        match dividend {
            Object::Int(x) => match divisor {
                Object::Int(y) => Object::Int(x / y),
                Object::Float(y) => Object::Float(x as f64 / y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: (x as f64) / y,
                    uncertainty: (x as f64) * u / (y * y),
                },
                _ => panic!("Can't divide Int by {}", divisor),
            },
            Object::Float(x) => match divisor {
                Object::Int(y) => Object::Float(x / (y as f64)),
                Object::Float(y) => Object::Float(x / y),
                Object::Uncertain {
                    value: y,
                    uncertainty: u,
                } => Object::Uncertain {
                    value: x / y,
                    uncertainty: x * u / (y * y),
                },
                _ => panic!("Can't divide Float by {}", divisor),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match divisor {
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
                _ => panic!("Can't divide Uncertain by {}", divisor),
            },
            Object::Column(dividend_data) => {
                let mut quotients: Vec<Object> = Vec::new();
                if let Object::Column(divisor_data) = divisor {
                    for (index, dividend) in dividend_data.iter().enumerate() {
                        quotients.push(Interpreter::divide(
                            dividend.clone(),
                            divisor_data[index].clone(),
                        ))
                    }
                } else {
                    for dividend in dividend_data {
                        quotients.push(Interpreter::add(dividend, divisor.clone()))
                    }
                }
                Object::Column(quotients)
            }
            _ => panic!("Can't divide {dividend}"),
        }
    }

    pub fn exponent(base: Object, power: Object) -> Object {
        match base {
            Object::Int(x) => match power {
                Object::Int(y) => Object::Int(x.pow(y as u32)),
                Object::Float(y) => Object::Float((x as f64).powf(y)),
                _ => panic!("Can't raise Int to {}", power),
            },
            Object::Float(x) => match power {
                Object::Int(y) => Object::Float(x.powf(y as f64)),
                Object::Float(y) => Object::Float(x.powf(y)),
                _ => panic!("Can't raise Float to {}", power),
            },
            Object::Uncertain {
                value: x,
                uncertainty: u1,
            } => match power {
                Object::Int(y) => Object::Uncertain {
                    value: x.powf(y as f64),
                    uncertainty: x.powf(y as f64) * (y as f64) * (u1 / x),
                },
                Object::Float(y) => Object::Uncertain {
                    value: x.powf(y),
                    uncertainty: x.powf(y) * y * (u1 / x),
                },
                _ => panic!("Can't raise Uncertain to {}", power),
            },
            _ => panic!("Can't exponentiate {base}"),
        }
    }

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

    pub fn greaterthan(operand1: &Object, operand2: &Object) -> Object {
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

    pub fn lessthanequal(operand1: &Object, operand2: &Object) -> Object {
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

    pub fn lessthan(operand1: &Object, operand2: &Object) -> Object {
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

    pub fn and(operand1: &Object, operand2: &Object) -> Object {
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

    pub fn or(operand1: &Object, operand2: &Object) -> Object {
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

    pub fn notequal(operand1: &Object, operand2: &Object) -> Object {
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

    pub fn equalequal(operand1: &Object, operand2: &Object) -> Object {
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

    pub fn not(eval_op: Object) -> Object {
        if let Object::Boolean(x) = eval_op {
            Object::Boolean(!x)
        } else {
            panic!("Expected bool, found {:?}", eval_op)
        }
    }

    pub fn negate(eval_op: &Object) -> Object {
        if let Object::Int(x) = eval_op {
            Object::Int(-x)
        } else if let Object::Float(x) = eval_op {
            Object::Float(-x)
        } else {
            panic!("Expected number, found {:?}", eval_op)
        }
    }

    fn make_uncertain(operand1: Object, operand2: Object) -> Object {
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

    pub fn call_function(&mut self, identifier: String, args: Vec<Object>) -> Object {
        if let Object::Function { params, block } = self.get_variable(identifier.clone()) {
            self.scopes.push(HashMap::new());
            for (index, param) in params.iter().enumerate() {
                let arg = args[index].clone();
                self.insert_top_scope(param.clone(), arg)
            }
            self.run_statement(*block);
            let result = self.get_variable(String::from("return"));
            self.scopes.pop();
            result
        } else {
            let check_std = self.run_fn_std(identifier.clone(), args.clone()); // Check if function exists in standard library
            if let Some(Object::Function { params, block }) = check_std.clone() {
                self.globals.insert(
                    identifier.clone(),
                    Object::Function {
                        params: params.clone(),
                        block: block.clone(),
                    },
                );
                self.scopes.push(HashMap::new());
                for (index, param) in params.iter().enumerate() {
                    let val = args[index].clone();
                    self.insert_top_scope(param.clone(), val)
                }
                self.run_statement(*block);
                let result = self.get_variable(String::from("return"));
                self.scopes.pop();
                result
            } else if let Some(x) = check_std {
                x
            } else {
                panic!("The variable {identifier} does not appear to be a function. Did you define it? Is it in a file you haven't imported?")
            }
        }
    }
}
