use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_env(_: &mut Evaluator, key: String, _: Span) -> Result<Value, Error> {
    match std::env::var(&key) {
        Ok(val) => Ok(Value::String(val)),
        Err(_) => Ok(Value::Null),
    }
}
