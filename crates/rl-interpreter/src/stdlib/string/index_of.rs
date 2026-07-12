use crate::interpreter::evaluator::Evaluator;

pub fn std_index_of(_: &mut Evaluator, string: String, sub: String) -> i64 {
    match string.find(&sub) {
        Some(i) => string[..i].chars().count() as i64,
        None => -1_i64,
    }
}
