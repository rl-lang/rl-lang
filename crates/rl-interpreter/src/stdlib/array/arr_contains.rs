use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_contains(
    eval: &mut Evaluator,
    array: Value,
    value: Value,
    span: Span,
) -> Result<bool, Error> {
    match array {
        Value::Values { items, .. } => Ok(items.contains(&value)),

        _ => Err(eval.err("arr_contains() accepts only arrays".to_string(), span)),
    }
}
