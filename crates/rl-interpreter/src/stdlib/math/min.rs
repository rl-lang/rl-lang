use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

pub fn std_min(_: &mut Evaluator, a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => vok!(vi!(a.min(b))),
        (Value::Float(a), Value::Float(b)) => vok!(vf!(a.min(b))),
        (a, b) => verr!(vs!(format!(
            "min expects a number, got ({}, {})",
            a.type_name(),
            b.type_name()
        ))),
    }
}
