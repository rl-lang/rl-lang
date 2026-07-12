use crate::evaluator::Evaluator;

pub fn func(eval: &mut Evaluator, weight: f64) -> bool {
    eval.rng.generate_random_bool(weight)
}
