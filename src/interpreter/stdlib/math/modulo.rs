use crate::{interpreter::evaluator::Evaluator, interpreter::values::Value, utils::errors::Error};

pub fn std_mod(_: &mut Evaluator, a: Value, b: Value) -> Result<Value, Error> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(a as f64 % b)),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a % b as f64)),
        (a, b) => Err(Error::init(
            format!(
                "mod() expects a number, got ({}, {})",
                a.type_name(),
                b.type_name()
            ),
            None,
            None,
        )),
    }
}
