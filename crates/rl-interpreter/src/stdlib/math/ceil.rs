use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

pub fn std_ceil(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => vok!(vi!(i)),
        Value::Float(f) => vok!(vf!(f.ceil())),
        other => verr!(vs!(format!(
            "ceil() expects a number, got {}",
            other.type_name()
        ))),
    }
}
