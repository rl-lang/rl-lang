use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_index_of(
    eval: &mut Evaluator,
    array: Value,
    value: Value,
    span: Span,
) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => match items.iter().position(|item| *item == value) {
            Some(pos) => Ok(Value::Integer(pos as i64)),
            None => Ok(Value::Integer(-1)),
        },
        _ => Err(eval.err("arr_index_of() accepts only arrays".to_string(), span)),
    }
}
