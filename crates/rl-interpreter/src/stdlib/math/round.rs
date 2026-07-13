use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

pub fn std_round(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => vok!(vi!(i)),
        Value::Float(f) => vok!(vf!(f.round())),
        other => verr!(vs!(format!(
            "round expects a number, got {}",
            other.type_name()
        ))),
    }
}
