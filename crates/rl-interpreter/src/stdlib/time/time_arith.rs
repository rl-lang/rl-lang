use crate::interpreter::evaluator::Evaluator;

// yes... useless... for now
// should add timestamp or time type later
pub fn time_add(_: &mut Evaluator, ts: i64, seconds: i64) -> i64 {
    ts + seconds
}

pub fn time_diff(_: &mut Evaluator, a: i64, b: i64) -> i64 {
    a - b
}
