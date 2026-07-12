use crate::evaluator::Evaluator;

pub fn std_euler_gamma(_: &mut Evaluator) -> f64 {
    std::f64::consts::EULER_GAMMA
}
