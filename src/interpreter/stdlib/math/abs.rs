use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

/// returns the absolute value of number
pub fn std_abs(_: &mut Evaluator, a: Value) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Integer(i.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        other => Err(Error::init(
            format!("abs() expects a number, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
