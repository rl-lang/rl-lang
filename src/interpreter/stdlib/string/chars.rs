use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_chars(_: &mut Evaluator, string: String) -> Value {
    let chars = string.chars().map(|c| Value::Char(c)).collect();
    Value::Values(chars)
}
