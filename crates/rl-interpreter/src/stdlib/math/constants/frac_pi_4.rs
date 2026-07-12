use crate::interpreter::evaluator::Evaluator;

pub fn std_frac_pi_4(_: &mut Evaluator) -> f64 {
    std::f64::consts::FRAC_PI_4
}
