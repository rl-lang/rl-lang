use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_char(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Char(_))
}
