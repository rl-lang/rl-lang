use crate::evaluator::Evaluator;

pub fn std_to_lower(_: &mut Evaluator, string: String) -> String {
    string.to_lowercase()
}
