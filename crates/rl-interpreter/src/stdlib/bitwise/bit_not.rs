use crate::{
    evaluator::Evaluator,
    stdlib::common::{vby, verr, vi, vok, vs},
    values::Value,
};

pub fn std_bit_not(_: &mut Evaluator, v: Value) -> Value {
    match v {
        Value::Byte(x) => vok!(vby!(!x)),
        Value::Integer(x) => vok!(vi!(!x)),
        _ => verr!(vs!("bit_not expects a byte or an int".to_string())),
    }
}
