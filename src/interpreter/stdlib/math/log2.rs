use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_log2(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => Value::Float((i as f64).log2()),
        Value::Float(f) => Value::Float(f.log2()),
        _ => {
            Error::init("".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
