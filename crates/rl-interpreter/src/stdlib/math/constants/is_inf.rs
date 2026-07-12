use crate::interpreter::evaluator::Evaluator;

pub fn std_is_inf(_: &mut Evaluator, x: f64) -> bool {
    x.is_infinite()
}
