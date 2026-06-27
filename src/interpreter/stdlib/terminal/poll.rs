use crate::interpreter::stdlib::common::{check_arity, extract_number};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::event::poll;
use std::time::Duration;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "term_poll", span)?;

    let ms = extract_number(args[0].clone(), "term_poll", span)?;

    let ready = poll(Duration::from_millis(ms))
        .map_err(|e| eval.err(format!("term_poll(): {}", e), span))?;
    Ok(Value::Bool(ready))
}
