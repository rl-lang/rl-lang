use crate::interpreter::evaluator::Evaluator;

pub fn std_atan(_: &mut Evaluator, x: f64) -> f64 {
    x.atan()
}
