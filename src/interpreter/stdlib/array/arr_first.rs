use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_first(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => match items.into_iter().next() {
            Some(v) => Ok(v),
            None => Err(Error::init(
                "arr_first() called on empty array".to_string(),
                None,
                None,
            )),
        },
        _ => Err(Error::init(
            "arr_first() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
