use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_floor(_: &mut Evaluator, a: Value) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Integer(i)),
        Value::Float(f) => Ok(Value::Float(f.floor())),
        other => Err(Error::init(
            format!("floor() expects a number, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
