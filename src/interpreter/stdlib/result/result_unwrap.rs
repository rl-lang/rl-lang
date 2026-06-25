use crate::{
    interpreter::{evaluator::Evaluator, stdlib::check_arity, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_unwrap(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "result_unwrap", span)?;
    match &args[0] {
        Value::Ok(inner) => Ok(*inner.clone()),
        Value::Err(v) => Err(eval.err(format!("result_unwrap: called on Err({})", v), span)),
        other => Err(eval.err(
            format!("result_unwrap: expected result, got {}", other.type_name()),
            span,
        )),
    }
}

pub fn std_unwrap_err(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "result_unwrap_err", span)?;
    match &args[0] {
        Value::Err(inner) => Ok(*inner.clone()),
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

pub fn std_unwrap_or(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 2, "result_unwrap_or", span)?;
    match &args[0] {
        Value::Ok(inner) => Ok(*inner.clone()),
        Value::Err(_) => Ok(args[1].clone()),
        other => Err(eval.err(
            format!(
                "result_unwrap_or: expected result, got {}",
                other.type_name()
            ),
            span,
        )),
    }
}
