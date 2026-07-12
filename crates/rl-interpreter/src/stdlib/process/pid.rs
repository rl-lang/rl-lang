use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_pid(_: &mut Evaluator, _: Vec<Value>, _: Span) -> Result<Value, Error> {
    Ok(Value::Integer(std::process::id() as i64))
}
