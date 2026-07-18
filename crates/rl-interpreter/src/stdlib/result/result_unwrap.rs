use rl_utils::{errors::Error, span::Span};

use crate::{evaluator::Evaluator, values::Value};

pub fn std_unwrap(eval: &mut Evaluator, v: Value, span: Span) -> Result<Value, Error> {
    match v {
        Value::Ok(inner) => Ok(*inner),
        Value::Err(v) => Err(eval.err(format!("result_unwrap: called on Err({})", v), span)),
        other => Err(eval.err(
            format!("result_unwrap: expected result, got {}", other.type_name()),
            span,
        )),
    }
}

pub fn std_unwrap_err(eval: &mut Evaluator, v: Value, span: Span) -> Result<Value, Error> {
    match v {
        Value::Err(inner) => Ok(*inner),
        Value::Ok(v) => Err(eval.err(format!("result_unwrap_err: called on ok({})", v), span)),
        other => Err(eval.err(
            format!(
                "result_unwrap_err: expected result, got {}",
                other.type_name()
            ),
            span,
        )),
    }
}

pub fn std_unwrap_or(eval: &mut Evaluator, v: Value, b: Value, span: Span) -> Result<Value, Error> {
    match v {
        Value::Ok(inner) => Ok(*inner),
        Value::Err(_) => Ok(b),
        other => Err(eval.err(
            format!(
                "result_unwrap_or: expected result, got {}",
                other.type_name()
            ),
            span,
        )),
    }
}
