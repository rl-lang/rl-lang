use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

pub fn std_mod(_: &mut Evaluator, a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => vok!(vi!(a % b)),
        (Value::Float(a), Value::Float(b)) => vok!(vf!(a % b)),
        (Value::Integer(a), Value::Float(b)) => vok!(vf!(a as f64 % b)),
        (Value::Float(a), Value::Integer(b)) => vok!(vf!(a % b as f64)),
        (a, b) => verr!(vs!(format!(
            "mod expects a number, got ({}, {})",
            a.type_name(),
            b.type_name()
        ))),
    }
}
