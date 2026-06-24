use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::errors::Error;

pub fn std_error_unwrap(_: &mut Evaluator, value: Value) -> Result<Value, Error> {
    match value {
        Value::Error(inner) => Ok(*inner),
        other => Err(crate::utils::errors::Error::at(
            crate::utils::errors::Reason::Runtime,
            format!("error_unwrap: expected error, got {}", other.type_name()),
            crate::utils::span::Span::default(),
        )),
    }
}
