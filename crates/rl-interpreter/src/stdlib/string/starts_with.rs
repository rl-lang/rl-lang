use crate::evaluator::Evaluator;

pub fn std_starts_with(_: &mut Evaluator, string: String, sub: String) -> bool {
    string.starts_with(&sub)
}
