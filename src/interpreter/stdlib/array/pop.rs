use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_pop(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values(v) => {
            let mut v = v;
            let v = v.pop().unwrap_or_else(|| Value::Null);
            Ok(v)
        }
        _ => Err(Error::init(
            "pop() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
