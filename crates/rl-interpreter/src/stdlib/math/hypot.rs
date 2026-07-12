use crate::interpreter::evaluator::Evaluator;

pub fn std_hypot(_: &mut Evaluator, x: f64, y: f64) -> f64 {
    x.hypot(y)
}
