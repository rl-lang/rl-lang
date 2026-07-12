use crate::interpreter::evaluator::Evaluator;

pub fn std_reverse(_: &mut Evaluator, string: String) -> String {
    string.chars().rev().collect()
}
