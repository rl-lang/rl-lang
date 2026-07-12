use crate::interpreter::evaluator::Evaluator;

pub fn std_to_upper(_: &mut Evaluator, string: String) -> String {
    string.to_uppercase()
}
