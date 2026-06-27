use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
};
use std::io::stdout;

pub fn std_term_begin_sync(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_begin_sync", span)?;

    execute!(stdout(), BeginSynchronizedUpdate)
        .map_err(|e| eval.err(format!("term_begin_sync(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_end_sync(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_end_sync", span)?;

    execute!(stdout(), EndSynchronizedUpdate)
        .map_err(|e| eval.err(format!("term_end_sync(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
