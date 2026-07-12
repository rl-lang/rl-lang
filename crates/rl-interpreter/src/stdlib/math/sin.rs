use crate::interpreter::evaluator::Evaluator;

pub fn std_sin(_: &mut Evaluator, x: f64) -> f64 {
    x.sin()
}
