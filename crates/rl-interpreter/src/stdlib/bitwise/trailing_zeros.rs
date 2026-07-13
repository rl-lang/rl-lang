use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_trailing_zeros(_: &mut Evaluator, v: Value) -> Value {
    match v {
        Value::Byte(x) => vok!(Value::Byte(u8::trailing_zeros(x) as u8)),
        Value::Integer(x) => vok!(Value::Integer(i64::trailing_zeros(x) as i64)),
        _ => verr!(vs!("trailing_zeros expects a byte or an int".to_string())),
    }
}
