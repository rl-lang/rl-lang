use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_is_empty(eval: &mut Evaluator, array: Value, span: Span) -> Result<bool, Error> {
    match array {
        Value::Values { items, .. } => {
            if items.is_empty() {
                return Ok(true);
            }
            Ok(false)
        }
        _ => Err(eval.err("arr_is_empty() accepts only arrays".to_string(), span)),
    }
}
