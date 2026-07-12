use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_remove(
    eval: &mut Evaluator,
    array: Value,
    index: i64,
    span: Span,
) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => {
            if index as usize >= items.len() {
                return Err(eval.err(format!("index out of bounds: {}", index), span));
            }
            let mut v = items;
            v.remove(index as usize);
            Ok(Value::Values {
                items_type,
                items: v,
            })
        }
        _ => Err(eval.err("arr_remove() accepts only arrays".to_string(), span)),
    }
}
