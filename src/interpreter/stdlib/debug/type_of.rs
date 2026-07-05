use crate::interpreter::{evaluator::Evaluator, stdlib::common::vs, values::Value};

pub fn func(_: &mut Evaluator, v: Value) -> Value {
    vs!(v.type_name().to_string())
}
