use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_bool(_: &mut Evaluator, value: Value) -> bool {
    match value {
        Value::Bool(_) => true,
        _ => false,
    }
}
