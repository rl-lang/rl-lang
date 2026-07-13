use crate::{
    evaluator::Evaluator,
    stdlib::common::{vby, verr, vi, vok, vs},
    values::Value,
};

pub fn std_bit_xor(_: &mut Evaluator, a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Byte(x), Value::Byte(y)) => vok!(vby!(x ^ y)),
        (Value::Integer(x), Value::Integer(y)) => vok!(vi!(x ^ y)),
        _ => verr!(vs!(
            "bit_xor expects (byte, byte) or (int, int) arguments".to_string()
        )),
    }
}
