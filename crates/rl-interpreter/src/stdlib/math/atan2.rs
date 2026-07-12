use crate::evaluator::Evaluator;

pub fn std_atan2(_: &mut Evaluator, x: f64, y: f64) -> f64 {
    y.atan2(x)
}
