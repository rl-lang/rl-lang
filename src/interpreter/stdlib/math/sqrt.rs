use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_sqrt(_: &mut Evaluator, a: Value) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Float((i as f64).sqrt())),
        Value::Float(f) => Ok(Value::Float(f.sqrt())),
        other => Err(Error::init(
            format!("round() expects a number, got {}", other.type_name(),),
            None,
            None,
        )),
    }
}
