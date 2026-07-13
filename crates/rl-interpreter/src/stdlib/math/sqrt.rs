use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vok, vs},
    values::Value,
};

pub fn std_sqrt(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => vok!(vf!((i as f64).sqrt())),
        Value::Float(f) => vok!(vf!(f.sqrt())),
        other => verr!(vs!(format!(
            "sqrt expects a number, got {}",
            other.type_name()
        ))),
    }
}
