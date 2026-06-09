use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_log2(_: &mut Evaluator, a: Value) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Float((i as f64).log2())),
        Value::Float(f) => Ok(Value::Float(f.log2())),
        other => Err(Error::init(
            format!("log2() expects a number, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
