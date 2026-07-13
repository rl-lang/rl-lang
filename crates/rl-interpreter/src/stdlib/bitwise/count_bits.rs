use crate::{
    evaluator::Evaluator,
    stdlib::common::{vby, verr, vi, vok, vs},
    values::Value,
};

pub fn std_count_bits(_: &mut Evaluator, v: Value) -> Value {
    match v {
        Value::Byte(x) => vok!(vby!(x.count_ones() as u8)),
        Value::Integer(x) => vok!(vi!(x.count_ones() as i64)),
        _ => verr!(vs!("count_bits expects a byte or an int".to_string())),
    }
}
