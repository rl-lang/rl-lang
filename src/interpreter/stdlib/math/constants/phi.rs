use crate::interpreter::evaluator::Evaluator;

pub fn std_phi(_: &mut Evaluator) -> f64 {
    std::f64::consts::GOLDEN_RATIO
}
