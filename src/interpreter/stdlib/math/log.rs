use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_log(_: &mut Evaluator, a: Value) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Float((i as f64).ln())),
        Value::Float(f) => Ok(Value::Float(f.ln())),
        other => Err(Error::init(
            format!("log() expects a number, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
