use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_int(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Integer(_))
}
