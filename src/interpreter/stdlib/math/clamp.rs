use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_clamp(
    eval: &mut Evaluator,
    value: Value,
    min: Value,
    max: Value,
    span: Span,
) -> Result<Value, Error> {
    match (value, min, max) {
        (Value::Integer(value), Value::Integer(low), Value::Integer(high)) => {
            Ok(Value::Integer(value.clamp(low, high)))
        }
        (Value::Float(value), Value::Float(low), Value::Float(high)) => {
            Ok(Value::Float(value.clamp(low, high)))
        }
        (value, min, max) => Err(eval.err(
            format!(
                "clamp() expects a number, got ({}, {}, {})",
                value.type_name(),
                min.type_name(),
                max.type_name()
            ),
            span,
        )),
    }
}
