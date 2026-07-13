use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vok, vs},
    values::Value,
};

pub fn std_log10(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => vok!(vf!((i as f64).log10())),
        Value::Float(f) => vok!(vf!(f.log10())),
        other => verr!(vs!(format!(
            "log10 expects a number, got {}",
            other.type_name()
        ))),
    }
}
