use crate::evaluator::Evaluator;

pub fn std_contains(_: &mut Evaluator, string: String, sub: String) -> bool {
    string.contains(&sub)
}
