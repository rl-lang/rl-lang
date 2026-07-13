use crate::{
    evaluator::Evaluator,
    stdlib::common::{vby, verr, vi, vok, vs},
    values::Value,
};

pub fn std_bit_shift_left(_: &mut Evaluator, a: Value, shift: Value) -> Value {
    match (a, shift) {
        (Value::Byte(x), Value::Byte(s)) => vok!(vby!(x << (s as u32))),
        (Value::Byte(x), Value::Integer(s)) => vok!(vby!(x << (s as u32))),
        (Value::Integer(x), Value::Byte(s)) => vok!(vi!(x << (s as u32))),
        (Value::Integer(x), Value::Integer(s)) => vok!(vi!(x << (s as u32))),
        _ => verr!(vs!(
            "bit_shift_left expects ((byte|int), (int|byte))".to_string()
        )),
    }
}
