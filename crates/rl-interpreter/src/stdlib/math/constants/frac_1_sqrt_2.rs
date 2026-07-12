use crate::interpreter::evaluator::Evaluator;

pub fn std_frac_1_sqrt_2(_: &mut Evaluator) -> f64 {
    std::f64::consts::FRAC_1_SQRT_2
}
