use crate::interpreter::evaluator::Evaluator;

pub fn std_ln_2(_: &mut Evaluator) -> f64 {
    std::f64::consts::LN_2
}
