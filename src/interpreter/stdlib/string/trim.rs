use crate::interpreter::evaluator::Evaluator;

pub fn std_trim(_: &mut Evaluator, string: String) -> String {
    string.trim().to_string()
}
