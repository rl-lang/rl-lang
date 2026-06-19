use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_push(
    eval: &mut Evaluator,
    array: Value,
    value: Value,
    span: Span,
) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
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
            v.push(value);
            Ok(Value::Values {
                items_type,
                items: v,
            })
        }
        _ => Err(eval.err("arr_push() accepts only arrays".to_string(), span)),
    }
}
