use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_ceil(_: &mut Evaluator, a: Value) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Integer(i)),
        Value::Float(f) => Ok(Value::Float(f.ceil())),
        other => Err(Error::init(
            format!("ceil() expects a number, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
