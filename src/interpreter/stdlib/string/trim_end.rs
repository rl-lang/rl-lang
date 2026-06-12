use crate::interpreter::evaluator::Evaluator;

pub fn std_trim_end(_: &mut Evaluator, string: String) -> String {
    string.trim_end().to_string()
}
