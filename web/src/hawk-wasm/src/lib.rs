use wasm_bindgen::prelude::*;

use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run(code: &str) {
    match hawk_interpreter::run::run(code.to_string(), HashMap::new(), true, |_| Ok("hi".to_owned()), |_, _| Ok(()), |message: String, _| alert(&format!("Warning: {message}")), |message: String, _| alert(&format!("Error: {message}")), |message: String| alert(&format!("{message}"))) {
        Ok(_) => (),
        Err((_, _)) => ()
    }
}
