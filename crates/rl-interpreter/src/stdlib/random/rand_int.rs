use crate::interpreter::evaluator::Evaluator;

pub fn func(eval: &mut Evaluator) -> i64 {
    eval.rng.generate_random_int_range(i64::MIN, i64::MAX)
}
