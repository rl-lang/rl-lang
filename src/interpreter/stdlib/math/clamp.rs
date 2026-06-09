use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_clamp(_: &mut Evaluator, value: Value, min: Value, max: Value) -> Result<Value, Error> {
    match (value, min, max) {
        (Value::Integer(value), Value::Integer(low), Value::Integer(high)) => {
            Ok(Value::Integer(value.clamp(low, high)))
        }
        (Value::Float(value), Value::Float(low), Value::Float(high)) => {
            Ok(Value::Float(value.clamp(low, high)))
        }
        (value, min, max) => Err(Error::init(
            format!(
                "clamp() expects a number, got ({}, {}, {})",
                value.type_name(),
                min.type_name(),
                max.type_name()
            ),
            None,
            None,
        )),
    }
}
