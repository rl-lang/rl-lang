use crate::interpreter::evaluator::Evaluator;

pub fn std_cos(_: &mut Evaluator, x: f64) -> f64 {
    x.cos()
}
