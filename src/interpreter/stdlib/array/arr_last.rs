use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_last(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => match items.into_iter().last() {
            Some(v) => Ok(v),
            None => Err(Error::init(
                "arr_last() called on empty array".to_string(),
                None,
                None,
            )),
        },
        _ => Err(Error::init(
            "arr_last() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
