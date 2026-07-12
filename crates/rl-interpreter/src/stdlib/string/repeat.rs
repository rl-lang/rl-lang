use crate::interpreter::evaluator::Evaluator;

pub fn std_repeat(_: &mut Evaluator, string: String, count: i64) -> String {
    string.repeat(count as usize)
}
