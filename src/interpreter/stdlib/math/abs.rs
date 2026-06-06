use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

/// returns the absolute value of number
pub fn std_abs(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => Value::Integer(i.abs()),
        Value::Float(f) => Value::Float(f.abs()),
        _ => {
            Error::init("".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
