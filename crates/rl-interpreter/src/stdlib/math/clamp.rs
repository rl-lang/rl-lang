use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

pub fn std_clamp(_: &mut Evaluator, value: Value, min: Value, max: Value) -> Value {
    match (value, min, max) {
        (Value::Integer(value), Value::Integer(low), Value::Integer(high)) => {
            vok!(vi!(value.clamp(low, high)))
        }
        (Value::Float(value), Value::Float(low), Value::Float(high)) => {
            vok!(vf!(value.clamp(low, high)))
        }
        (value, min, max) => verr!(vs!(format!(
            "clamp expects a number, got ({}, {}, {})",
            value.type_name(),
            min.type_name(),
            max.type_name()
        ))),
    }
}
