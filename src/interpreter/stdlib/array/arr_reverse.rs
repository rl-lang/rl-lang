use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_reverse(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            let mut items = items;
            items.reverse();
            Ok(Value::Values { items_type, items })
        }
        _ => Err(eval.err("arr_reverse() accepts only arrays".to_string(), span)),
    }
}
