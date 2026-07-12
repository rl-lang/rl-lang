use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_count(eval: &mut Evaluator, array: Value, span: Span) -> Result<i64, Error> {
    match array {
        Value::Values { items, .. } => Ok(items.len() as i64),
        _ => Err(eval.err("arr_is_empty() accepts only arrays".to_string(), span)),
    }
}
