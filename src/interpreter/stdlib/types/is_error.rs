use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_error(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Error(_))
}
