use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_flatten(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => {
            if items.is_empty() {
                return Err(eval.err("arr_flatten() called on empty array".to_string(), span));
            }

            Ok(Value::Values {
                items_type,
                items: items
                    .into_iter()
                    .flat_map(|v| {
                        if let Value::Values { items, .. } = v {
                            items
                        } else {
                            vec![v]
                        }
                    })
                    .collect(),
            })
        }
        _ => Err(eval.err("arr_flatten() accepts only arrays".to_string(), span)),
    }
}
