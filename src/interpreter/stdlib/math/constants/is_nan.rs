use crate::interpreter::evaluator::Evaluator;

pub fn std_is_nan(_: &mut Evaluator, x: f64) -> bool {
    x.is_nan()
}
