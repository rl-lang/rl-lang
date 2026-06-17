use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_count(_: &mut Evaluator, array: Value) -> Result<i64, Error> {
    match array {
        Value::Values { items, .. } => Ok(items.len() as i64),
        _ => Err(Error::init(
            "arr_is_empty() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
