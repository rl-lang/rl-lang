use crate::interpreter::evaluator::Evaluator;

pub fn std_cotains(_: &mut Evaluator, string: String, sub: String) -> bool {
    string.contains(&sub)
}
