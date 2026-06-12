use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_push(_: &mut Evaluator, array: Value, value: Value) -> Result<Value, Error> {
    match array {
        Value::Values(v) => {
            let mut v = v;
            v.push(value);
            Ok(Value::Values(v))
        }
        _ => Err(Error::init(
            "push() accepts only arrays and values".to_string(),
            None,
            None,
        )),
    }
}
