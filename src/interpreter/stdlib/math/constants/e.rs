use crate::interpreter::evaluator::Evaluator;

pub fn std_e(_: &mut Evaluator) -> f64 {
    std::f64::consts::E
}
