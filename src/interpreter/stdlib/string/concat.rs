use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_concat(_: &mut Evaluator, args: Vec<Value>, _: Span) -> Result<Value, Error> {
    Ok(Value::String(
        args.iter().map(|a| a.to_string()).collect::<String>(),
    ))
}
