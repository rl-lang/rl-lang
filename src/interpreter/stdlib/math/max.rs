use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_max(_: &mut Evaluator, a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a.max(b)),
        (Value::Float(a), Value::Float(b)) => Value::Float(a.max(b)),
        _ => {
            Error::init("".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
