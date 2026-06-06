use crate::{interpreter::evaluator::Evaluator, interpreter::values::Value, utils::errors::Error};

pub fn std_pow(_: &mut Evaluator, base: Value, exponent: Value) -> Value {
    match (base, exponent) {
        (Value::Integer(a), Value::Integer(b)) => {
            let a = a as i32;
            let b = b as u32;
            Value::Integer(a.pow(b) as i64)
        }
        (Value::Integer(a), Value::Float(b)) => Value::Float((a as f64).powf(b)),
        (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(b)),
        (Value::Float(a), Value::Integer(b)) => Value::Float(a.powi(b as i32)),
        _ => {
            Error::init(
                "only integers and floats are supported".to_string(),
                None,
                None,
            )
            .print_error();
            unreachable!()
        }
    }
}
