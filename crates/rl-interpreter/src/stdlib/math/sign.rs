use crate::interpreter::evaluator::Evaluator;

pub fn std_sign(_: &mut Evaluator, x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}
