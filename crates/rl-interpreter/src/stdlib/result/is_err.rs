use crate::evaluator::Evaluator;
use crate::stdlib::common::{vb, vok};
use crate::values::Value;

pub fn func(_: &mut Evaluator, value: Value) -> Value {
    vok!(vb!(value.is_err()))
}
