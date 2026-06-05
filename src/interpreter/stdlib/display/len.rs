use crate::{interpreter::evaluator::Evaluator, interpreter::values::Value, utils::errors::Error};

pub fn std_len(_: &mut Evaluator, v: Value) -> i64 {
    match v {
        Value::Values(items) => items.len() as i64,
        Value::String(s) => s.len() as i64,
        _ => {
            Error::init("len() expects an array or string".to_string(), None, None)
                .print_error();
            unreachable!()
        }
    }
}
