use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_unique(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            let mut seen = Vec::new();
            for item in items {
                if !seen.contains(&item) {
                    seen.push(item);
                }
            }
            Ok(Value::Values {
                items_type,
                items: seen,
            })
        }
        _ => Err(eval.err("arr_unique() accepts only arrays".to_string(), span)),
    }
}
