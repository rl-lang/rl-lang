use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_max(_: &mut Evaluator, a: Value, b: Value) -> Result<Value, Error> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a.max(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(b))),
        (a, b) => Err(Error::init(
            format!(
                "max() expects a number, got ({}, {})",
                a.type_name(),
                b.type_name()
            ),
            None,
            None,
        )),
    }
}
