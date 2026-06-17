use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_contains(_: &mut Evaluator, array: Value, value: Value) -> Result<bool, Error> {
    match array {
        Value::Values { items, .. } => {
            if items.contains(&value) {
                return Ok(true);
            }
            Ok(false)
        }
        _ => Err(Error::init(
            "arr_contains() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
