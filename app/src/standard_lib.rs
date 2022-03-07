use crate::Object;
use crate::standard_lib_rs::get_std_rs_fn;
use crate::standard_lib_hawk::get_std_hawk_fn;

pub fn run_fn_std(identifier: String) -> Option<Object> {
    if let Some(x) = get_std_rs_fn(identifier.clone()) {
        Some(x)
    } else if let Some(x) = get_std_hawk_fn(identifier) {
        Some(x)
    } else {
        None
    }
}