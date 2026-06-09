use crate::interpreter::evaluator::Evaluator;

pub fn std_factorial(_: &mut Evaluator, x: i64) -> i64 {
    (1..=x).product()
}
