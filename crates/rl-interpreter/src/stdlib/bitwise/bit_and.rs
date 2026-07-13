use crate::{
    evaluator::Evaluator,
    stdlib::common::{vby, verr, vi, vok, vs},
    values::Value,
};

pub fn std_bit_and(_: &mut Evaluator, a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Byte(x), Value::Byte(y)) => vok!(vby!(x & y)),
        (Value::Integer(x), Value::Integer(y)) => vok!(vi!(x & y)),
        (Value::Byte(x), Value::Integer(y)) => vok!(vi!(x as i64 & y)),
        (Value::Integer(x), Value::Byte(y)) => vok!(vi!(x & y as i64)),
        _ => verr!(vs!("bit_and expects byte or integer arguments".to_string())),
    }
}
