use crate::{
    evaluator::Evaluator,
    stdlib::common::{vby, verr, vi, vok, vs},
    values::Value,
};

pub fn std_leading_zeros(_: &mut Evaluator, v: Value) -> Value {
    match v {
        Value::Byte(x) => vok!(vby!(u8::leading_zeros(x) as u8)),
        Value::Integer(x) => vok!(vi!(i64::leading_zeros(x) as i64)),
        _ => verr!(vs!("leading_zeros expects a byte or an int".to_string())),
    }
}
