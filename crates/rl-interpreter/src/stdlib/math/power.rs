use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};

pub fn std_pow(_: &mut Evaluator, base: Value, exponent: Value) -> Value {
    match (base, exponent) {
        (Value::Integer(a), Value::Integer(b)) => {
            let b = b as u32;
            vok!(vi!(a.pow(b)))
        }
        (Value::Integer(a), Value::Float(b)) => vok!(vf!((a as f64).powf(b))),
        (Value::Float(a), Value::Float(b)) => vok!(vf!(a.powf(b))),
        (Value::Float(a), Value::Integer(b)) => vok!(vf!(a.powi(b as i32))),
        _ => verr!(vs!("pow expects numeric arguments".to_string())),
    }
}
