use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_bytes(_: &mut Evaluator, string: String) -> Value {
    let bytes = string.bytes().map(|b| Value::Integer(b as i64)).collect();
    Value::Values(bytes)
}
