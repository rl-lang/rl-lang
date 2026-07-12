use crate::interpreter::evaluator::Evaluator;

pub fn std_trim_start(_: &mut Evaluator, string: String) -> String {
    string.trim_start().to_string()
}
