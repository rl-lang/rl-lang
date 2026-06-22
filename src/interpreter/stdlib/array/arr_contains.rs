use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_contains(
    eval: &mut Evaluator,
    array: Value,
    value: Value,
    span: Span,
) -> Result<bool, Error> {
    match array {
        Value::Values { items, items_type } => {
            let needle = Evaluator::coerce_array_type(value, &items_type);
            Ok(items.contains(&needle))
        }
        _ => Err(eval.err("arr_contains() accepts only arrays".to_string(), span)),
    }
}
