use crate::interpreter::evaluator::Evaluator;

pub fn std_tan(_: &mut Evaluator, x: f64) -> f64 {
    x.tan()
}
