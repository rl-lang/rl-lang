use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_string(_: &mut Evaluator, value: Value) -> bool {
    match value {
        Value::String(_) => true,
        _ => false,
    }
}
