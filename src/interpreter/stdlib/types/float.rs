use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_float(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Float(_))
}
