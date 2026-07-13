use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

pub fn std_max(_: &mut Evaluator, a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => vok!(vi!(a.max(b))),
        (Value::Float(a), Value::Float(b)) => vok!(vf!(a.max(b))),
        (a, b) => verr!(vs!(format!(
            "max expects a number, got ({}, {})",
            a.type_name(),
            b.type_name()
        ))),
    }
}
