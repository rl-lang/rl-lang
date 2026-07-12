use crate::evaluator::Evaluator;

pub fn func(eval: &mut Evaluator) -> bool {
    let rand_float = eval.rng.generate_random_float();
    eval.rng.generate_random_bool(rand_float)
}
