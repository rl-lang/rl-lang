use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_round(_: &mut Evaluator, a: Value) -> Value {
    match a {
        Value::Integer(i) => Value::Integer(i),
        Value::Float(f) => Value::Float(f.round()),
        _ => {
            Error::init("".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
