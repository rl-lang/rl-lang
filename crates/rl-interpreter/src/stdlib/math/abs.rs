use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

/// returns the absolute value of number
pub fn std_abs(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => vok!(vi!(i.abs())),
        Value::Float(f) => vok!(vf!(f.abs())),
        other => verr!(vs!(format!(
            "abs() expects a number, got {}",
            other.type_name()
        ))),
    }
}
