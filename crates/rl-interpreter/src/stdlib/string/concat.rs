use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_concat(_: &mut Evaluator, args: Vec<Value>, _: Span) -> Result<Value, Error> {
    Ok(Value::String(
        args.iter().map(|a| a.to_string()).collect::<String>(),
    ))
}
