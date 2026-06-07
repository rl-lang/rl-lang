use crate::interpreter::evaluator::Evaluator;

pub fn std_sqrt_2(_: &mut Evaluator) -> f64 {
    std::f64::consts::SQRT_2
}
