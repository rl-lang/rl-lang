use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_slice(
    eval: &mut Evaluator,
    array: Value,
    start: i64,
    end: i64,
    span: Span,
) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            let start = start as usize;
            let end = end as usize;
            if start > items.len() || end > items.len() {
                return Err(eval.err(
                    format!(
                        "slice index out of bounds: {}..{} (len {})",
                        start,
                        end,
                        items.len()
                    ),
                    span,
                ));
            }
            if start > end {
                return Err(eval.err(
                    format!("slice start {} is greater than end {}", start, end),
                    span,
                ));
            }
            Ok(Value::Values {
                items_type,
                items: items[start..end].to_vec(),
            })
        }
        _ => Err(eval.err("arr_slice() accepts only arrays".to_string(), span)),
    }
}
