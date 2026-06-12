use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_char(_: &mut Evaluator, value: Value) -> bool {
    match value {
        Value::Char(_) => true,
        _ => false,
    }
}
