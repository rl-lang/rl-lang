use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_is_empty(_: &mut Evaluator, array: Value) -> Result<bool, Error> {
    match array {
        Value::Values { items, .. } => {
            if items.is_empty() {
                return Ok(true);
            }
            Ok(false)
        }
        _ => Err(Error::init(
            "arr_is_empty() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
