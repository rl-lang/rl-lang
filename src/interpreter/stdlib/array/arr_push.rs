use crate::{
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
            if !Evaluator::types_compatible(&val_type, &items_type) {
                return Err(eval.err(
                    format!(
                        "type mismatch: array expects {:?}, cannot push {:?}",
                        items_type, val_type
                    ),
                    span,
                ));
            }
            let value = Evaluator::coerce_array_type(value, &items_type);
            let mut v = items;
            v.push(value);
            Ok(Value::Values {
                items_type,
                items: v,
            })
        }
        other => Err(eval.err(
            format!("arr_push() accepts only arrays found {}", other.type_name()).to_string(),
            span,
        )),
    }
}
