use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_bool(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Bool(_))
}
