use crate::evaluator::Evaluator;

pub fn std_acos(_: &mut Evaluator, x: f64) -> f64 {
    x.acos()
}
