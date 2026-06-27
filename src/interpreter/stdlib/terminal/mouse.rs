use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::io::stdout;

pub fn std_term_enable_mouse(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_enable_mouse", span)?;

    execute!(stdout(), EnableMouseCapture)
        .map_err(|e| eval.err(format!("term_enable_mouse(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_disable_mouse(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_disable_mouse", span)?;

    execute!(stdout(), DisableMouseCapture)
        .map_err(|e| eval.err(format!("term_disable_mouse(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
