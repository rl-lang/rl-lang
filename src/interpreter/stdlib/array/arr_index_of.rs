use std::ops::Index;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_index_of(
    eval: &mut Evaluator,
    array: Value,
    index: i64,
    span: Span,
) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => {
            if index as usize >= items.len() {
                return Err(eval.err(format!("index out of bounds: {}", index), span));
            }
            let item = items.index(index as usize);
            Ok(item.clone())
        }
        _ => Err(eval.err("arr_is_empty() accepts only arrays".to_string(), span)),
    }
}
