use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_clamp(_: &mut Evaluator, value: Value, min: Value, max: Value) -> Value {
    match (value, min, max) {
        (Value::Integer(value), Value::Integer(low), Value::Integer(high)) => {
            Value::Integer(value.clamp(low, high))
        }
        (Value::Float(value), Value::Float(low), Value::Float(high)) => {
            Value::Float(value.clamp(low, high))
        }
        _ => {
            Error::init("".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
