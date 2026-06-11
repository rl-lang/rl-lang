use crate::interpreter::evaluator::Evaluator;

pub fn std_ends_with(_: &mut Evaluator, string: String, sub: String) -> bool {
    string.ends_with(&sub)
}
