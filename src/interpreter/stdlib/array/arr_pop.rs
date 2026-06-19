use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_pop(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values {
            items_type,
            mut items,
        } => {
            if items.is_empty() {
                return Err(eval.err("arr_pop() called on empty array".to_string(), span));
            }
            items.pop();
            Ok(Value::Values { items_type, items })
        }
        _ => Err(eval.err("arr_pop() accepts only arrays".to_string(), span)),
    }
}
