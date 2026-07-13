use crate::{evaluator::Evaluator, stdlib::common::vi, values::Value};

pub fn std_pid(_: &mut Evaluator) -> Value {
    vi!(std::process::id() as i64)
}
