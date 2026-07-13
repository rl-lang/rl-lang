use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vok, vs},
    values::Value,
};

pub fn std_log2(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => vok!(vf!((i as f64).log2())),
        Value::Float(f) => vok!(vf!(f.log2())),
        other => verr!(vs!(format!(
            "log2 expects a number, got {}",
            other.type_name()
        ))),
    }
}
