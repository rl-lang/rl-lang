use crate::interpreter::evaluator::Evaluator;

pub fn std_frac_2_pi(_: &mut Evaluator) -> f64 {
    std::f64::consts::FRAC_2_PI
}
