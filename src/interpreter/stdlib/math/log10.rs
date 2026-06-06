use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_log10(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => Value::Float((i as f64).log10()),
        Value::Float(f) => Value::Float(f.log10()),
        _ => {
            Error::init("".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
