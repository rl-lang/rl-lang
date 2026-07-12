use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_floor(eval: &mut Evaluator, a: Value, span: Span) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Integer(i)),
        Value::Float(f) => Ok(Value::Float(f.floor())),
        other => Err(eval.err(
            format!("floor() expects a number, got {}", other.type_name()),
            span,
        )),
    }
}
