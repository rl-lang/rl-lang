use crate::interpreter::evaluator::Evaluator;

pub fn std_asin(_: &mut Evaluator, x: f64) -> f64 {
    x.asin()
}
