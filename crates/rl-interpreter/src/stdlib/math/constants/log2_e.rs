use crate::interpreter::evaluator::Evaluator;

pub fn std_log2_e(_: &mut Evaluator) -> f64 {
    std::f64::consts::LOG2_E
}
