use crate::Interpreter;
use hawk_common::object::Object;

pub mod standard_lib_hawk;
mod standard_lib_rs;

use standard_lib_hawk::get_std_hawk_fn;

impl Interpreter {
    pub fn run_fn_std(&mut self, identifier: String, args: Vec<Object>) -> Result<Option<Object>, (String, usize)> {
        if let Some(x) = self.get_std_rs_fn(identifier.clone(), args)? {
            Ok(Some(x))
        } else if let Some(x) = get_std_hawk_fn(identifier) {
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }
}
