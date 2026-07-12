use crate::evaluator::Evaluator;

pub fn std_pi(_: &mut Evaluator) -> f64 {
    std::f64::consts::PI
}
