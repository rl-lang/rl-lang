use crate::evaluator::Evaluator;

pub fn std_replace(_: &mut Evaluator, string: String, from: String, to: String) -> String {
    string.replace(&from, &to)
}
