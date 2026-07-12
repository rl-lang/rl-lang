use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_insert(
    eval: &mut Evaluator,
    array: Value,
    value: Value,
    index: i64,
    span: Span,
) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            if index as usize >= items.len() {
                return Err(eval.err(format!("index out of bounds: {}", index), span));
            }

            let val_type = Evaluator::infer_type(&value, false);
            if val_type != items_type && val_type != TypeAnnotation::Null {
                return Err(eval.err(
                    format!(
                        "type mismatch: array expects {:?}, cannot push {:?}",
                        items_type, val_type
                    ),
                    span,
                ));
            }
            let mut v = items;
            v.insert(index as usize, value);
            Ok(Value::Values {
                items_type,
                items: v,
            })
        }
        _ => Err(eval.err(
            "arr_insert() accepts only arrays and values".to_string(),
            span,
        )),
    }
}
