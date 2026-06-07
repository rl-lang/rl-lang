use crate::interpreter::evaluator::Evaluator;

pub fn std_tua(_: &mut Evaluator) -> f64 {
    std::f64::consts::TAU
}
