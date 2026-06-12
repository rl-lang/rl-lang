use crate::interpreter::evaluator::Evaluator;

pub fn std_tau(_: &mut Evaluator) -> f64 {
    std::f64::consts::TAU
}
