use crate::interpreter::evaluator::Evaluator;

pub fn func(eval: &mut Evaluator) -> f64 {
    eval.rng.generate_random_float()
}
