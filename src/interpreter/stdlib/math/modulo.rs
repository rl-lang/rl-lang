use crate::{interpreter::evaluator::Evaluator, interpreter::values::Value, utils::errors::Error};

pub fn std_mod(_: &mut Evaluator, a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a % b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
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
