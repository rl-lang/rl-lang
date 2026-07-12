use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

/// returns the absolute value of number
pub fn std_abs(eval: &mut Evaluator, a: Value, span: Span) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Integer(i.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        other => Err(eval.err(
            format!("abs() expects a number, got {}", other.type_name()),
            span,
        )),
    }
}
