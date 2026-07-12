use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_last(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => match items.into_iter().last() {
            Some(v) => Ok(v),
            None => Err(eval.err("arr_last() called on empty array".to_string(), span)),
        },
        _ => Err(eval.err("arr_last() accepts only arrays".to_string(), span)),
    }
}
