use crate::interpreter::evaluator::Evaluator;

pub fn std_log2_10(_: &mut Evaluator) -> f64 {
    std::f64::consts::LOG2_10
}
