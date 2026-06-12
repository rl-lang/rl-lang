use crate::interpreter::evaluator::Evaluator;

pub fn std_is_empty(_: &mut Evaluator, string: String) -> bool {
    string.is_empty()
}
