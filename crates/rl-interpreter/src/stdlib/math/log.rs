use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vok, vs},
    values::Value,
};

pub fn std_log(_: &mut Evaluator, a: Value, base: Value) -> Value {
    match (a, base) {
        (Value::Integer(i), Value::Integer(base)) => vok!(vf!((i as f64).log(base as f64))),
        (Value::Float(f), Value::Float(base)) => vok!(vf!(f.log(base))),
        (Value::Float(f), Value::Integer(base)) => vok!(vf!(f.log(base as f64))),
        (Value::Integer(i), Value::Float(base)) => vok!(vf!((i as f64).log(base))),

        (a, base) => verr!(vs!(format!(
            "log expects a number, got ({}, {})",
            a.type_name(),
            base.type_name()
        ))),
    }
}
