use crate::interpreter::{evaluator::Evaluator, stdlib::common::vs, values::Value};

pub fn func(_: &mut Evaluator) -> Value {
    vs!(env!("CARGO_PKG_VERSION").to_string())
}
