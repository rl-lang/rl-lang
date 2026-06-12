use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_string(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::String(_))
}
