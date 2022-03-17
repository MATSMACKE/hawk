use crate::Object;
use crate::eval::Interpreter;
use crate::standard_lib_hawk::get_std_hawk_fn;

impl Interpreter {
    pub fn run_fn_std(&mut self, identifier: String, args: Vec<Object>) -> Option<Object> {
        if let Some(x) = self.get_std_rs_fn(identifier.clone(), args) {
            Some(x)
        } else if let Some(x) = get_std_hawk_fn(identifier) {
            Some(x)
        } else {
            None
        }
    }
}