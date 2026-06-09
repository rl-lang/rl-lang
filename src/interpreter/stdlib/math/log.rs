use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_log(_: &mut Evaluator, a: Value, base: Value) -> Result<Value, Error> {
    match (a, base) {
        (Value::Integer(i), Value::Integer(base)) => Ok(Value::Float((i as f64).log(base as f64))),
        (Value::Float(f), Value::Float(base)) => Ok(Value::Float(f.log(base))),
        (Value::Float(f), Value::Integer(base)) => Ok(Value::Float(f.log(base as f64))),
        (Value::Integer(i), Value::Float(base)) => Ok(Value::Float((i as f64).log(base))),

        (a, base) => Err(Error::init(
            format!(
                "log() expects a number, got ({}, {})",
                a.type_name(),
                base.type_name()
            ),
            None,
            None,
        )),
    }
}
