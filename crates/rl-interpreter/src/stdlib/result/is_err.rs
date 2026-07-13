use crate::evaluator::Evaluator;
use crate::values::Value;

pub fn func(_: &mut Evaluator, value: Value) -> bool {
    value.is_err()
}
