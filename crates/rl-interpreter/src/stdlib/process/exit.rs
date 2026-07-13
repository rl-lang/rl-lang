use crate::{evaluator::Evaluator, values::Value};

pub fn std_exit(_: &mut Evaluator, code: i64) -> Value {
    std::process::exit(code as i32);
}
