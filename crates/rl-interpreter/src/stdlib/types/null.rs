use crate::{evaluator::Evaluator, values::Value};

pub fn func(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Null)
}
