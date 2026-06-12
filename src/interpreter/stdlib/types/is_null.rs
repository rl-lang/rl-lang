use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_null(_: &mut Evaluator, value: Value) -> bool {
    match value {
        Value::Null => true,
        _ => false,
    }
}
