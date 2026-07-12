use crate::evaluator::Evaluator;

pub fn std_log10_e(_: &mut Evaluator) -> f64 {
    std::f64::consts::LOG10_E
}
