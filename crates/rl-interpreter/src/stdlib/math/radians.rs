use crate::evaluator::Evaluator;

pub fn std_radians(_: &mut Evaluator, x: f64) -> f64 {
    x.to_radians()
}
