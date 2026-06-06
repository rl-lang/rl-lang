use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_sqrt(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => Value::Float((i as f64).sqrt()),
        Value::Float(f) => Value::Float(f.sqrt()),
        _ => {
            Error::init("".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
