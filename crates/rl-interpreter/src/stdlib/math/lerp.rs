use crate::interpreter::evaluator::Evaluator;

pub fn std_lerp(_: &mut Evaluator, x: f64, y: f64, t: f64) -> f64 {
    x + (y - x) * t
}
