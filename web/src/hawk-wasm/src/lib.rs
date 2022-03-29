use wasm_bindgen::prelude::*;

use std::collections::HashMap;

#[wasm_bindgen(module = "/hawk_interface.ts")]
extern "C" {
    fn print(message: &str);
    fn warn(message: &str);
    fn error(message: &str);
    fn writefile(name: &str, content: &str);
    fn readfile(name: &str) -> String;
}

#[wasm_bindgen]
pub fn run(code: &str, in_repl: bool) {
    let filein_fn = |name: String| {
        Ok(readfile(&name))
    };

    let fileout_fn = |name: String, content: String| {
        writefile(&name, &content);
        Ok(())
    };

    let warn_fn = |message: String, line: usize| {
        if line == 0 {
            warn(&format!("Warning: {message}"));
        } else {
            warn(&format!("Warning on line {line}: {message}"));
        }
    };

    let err_fn = |message: String, line: usize| {
        if line == 0 {
            error(&format!("Error: {message}"));
        } else {
            error(&format!("Error on line {line}: {message}"));
        }
    };

    let output_fn = |message: String| print(&message);


    match hawk_interpreter::run::run(code.to_string(), HashMap::new(), in_repl, filein_fn, fileout_fn, warn_fn, err_fn, output_fn) {
        Ok(_) => (),
        Err((message, line)) => (err_fn(message, line))
    }
}
