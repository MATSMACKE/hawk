use std::collections::HashMap;
use std::vec;

use crate::object::Object;
use crate::run;

#[test]
fn end_to_end_let_int() {
    let result = run::run(String::from("let var = 5"), HashMap::new());
    let expected_var = result.get("var");

    if let Some(&Object::Int(x)) = expected_var {
        assert!(x == 5)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_let_float() {
    let result = run::run(String::from("let var = 5.3"), HashMap::new());
    let expected_var = result.get("var");

    if let Some(&Object::Float(x)) = expected_var {
        assert_eq!(x, 5.3)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_let_string() {
    let result = run::run(String::from("let var = \"3 + 5.3\""), HashMap::new());
    let expected_var = result.get("var");

    if let Some(Object::String(x)) = expected_var {
        assert_eq!(*x, String::from("3 + 5.3"))
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_let_array() {
    let result = run::run(String::from("let var = [4, 6, 87, ,2]"), HashMap::new());
    let expected_var = result.get("var");

    use Object::{Array, Int, Null};

    if let Some(Array(x)) = expected_var {
        assert_eq!(*x, vec![Int(4), Int(6), Int(87), Null, Int(2)])
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_let_boolean() {
    let result = run::run(String::from("let var = true"), HashMap::new());
    let expected_var = result.get("var");

    use Object::Boolean;

    if let Some(&Boolean(x)) = expected_var {
        assert!(x)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_let_uncertain() {
    let result = run::run(String::from("let var = 1 ± 0.1"), HashMap::new());
    let expected_var = result.get("var");

    use Object::Uncertain;

    if let Some(&Uncertain { value, uncertainty }) = expected_var {
        assert_eq!(value, 1.);
        assert_eq!(uncertainty, 0.1)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_ops_int() {
    let result = run::run(
        String::from("let var = 3 * (1 + 2) + 4 / 2 ^ 2"),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    if let Some(&Object::Int(x)) = expected_var {
        assert!(x == 10)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_ops_float() {
    let result = run::run(
        String::from("let var = 5.3 * 2.1 / (5.7 - 0.9) ^ 0.2"),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    if let Some(&Object::Float(x)) = expected_var {
        assert_eq!(x, 8.132927799776015)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_ops_string() {
    let result = run::run(
        String::from("let var = \"3 + 5.3\" + \"45\""),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    if let Some(Object::String(x)) = expected_var {
        assert_eq!(*x, String::from("3 + 5.345"))
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_ops_uncertain() {
    let result = run::run(
        String::from("let var = (1 ± 0.1 + 2 ± 0.4) * 2 ± 0.3"),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    use Object::Uncertain;

    if let Some(&Uncertain { value, uncertainty }) = expected_var {
        assert_eq!(value, 6.);
        assert_eq!(uncertainty, 1.9)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_ops_boolean() {
    let result = run::run(
        String::from("let var = true or false and not false"),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    use Object::Boolean;

    if let Some(&Boolean(x)) = expected_var {
        assert!(x)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_std_mod() {
    let result = run::run(
        String::from("let var = mod(45, 25)"),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    if let Some(&Object::Int(x)) = expected_var {
        assert!(x == 20)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_std_sin_0() {
    let result = run::run(
        String::from("let var = sin(0)"),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    if let Some(&Object::Float(x)) = expected_var {
        assert!(x == 0.)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_std_sin_pi() {
    let result = run::run(
        String::from("let var = sin(pi() / 2)"),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    if let Some(&Object::Float(x)) = expected_var {
        assert!(x == 1.)
    } else {
        assert!(false)
    }
}

#[test]
fn end_to_end_fn() {
    let result = run::run(
        String::from("
            function test(x) {
                return x + 1
            }

            let var = test(3)
        "),
        HashMap::new(),
    );
    let expected_var = result.get("var");

    if let Some(&Object::Int(x)) = expected_var {
        assert!(x == 4)
    } else {
        assert!(false)
    }
}