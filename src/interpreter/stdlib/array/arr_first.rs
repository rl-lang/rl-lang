use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_first(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => match items.into_iter().next() {
            Some(v) => Ok(v),
            None => Err(eval.err("arr_first() called on empty array".to_string(), span)),
        },
        _ => Err(eval.err("arr_first() accepts only arrays".to_string(), span)),
    }
}
