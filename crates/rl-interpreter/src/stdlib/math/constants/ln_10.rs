use crate::evaluator::Evaluator;

pub fn std_ln_10(_: &mut Evaluator) -> f64 {
    std::f64::consts::LN_10
}
