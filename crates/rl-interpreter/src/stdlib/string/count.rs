use crate::interpreter::evaluator::Evaluator;

pub fn std_count(_: &mut Evaluator, string: String, to_count: String) -> i64 {
    string.matches(&to_count).count() as i64
}
