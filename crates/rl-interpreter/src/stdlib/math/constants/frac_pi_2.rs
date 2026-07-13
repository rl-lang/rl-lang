use crate::evaluator::Evaluator;

pub fn std_frac_pi_2(_: &mut Evaluator) -> f64 {
    std::f64::consts::FRAC_PI_2
}
