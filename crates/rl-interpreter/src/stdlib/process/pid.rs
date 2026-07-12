use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_pid(_: &mut Evaluator, _: Vec<Value>, _: Span) -> Result<Value, Error> {
    Ok(Value::Integer(std::process::id() as i64))
}
