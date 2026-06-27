use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    terminal::{DisableLineWrap, EnableLineWrap},
};
use std::io::stdout;

pub fn std_term_enable_wrap(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_enable_wrap", span)?;

    execute!(stdout(), EnableLineWrap)
        .map_err(|e| eval.err(format!("term_enable_wrap(): {}", e), span))?;
    Ok(Value::Null)
}

pub fn std_term_disable_wrap(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_disable_wrap", span)?;

    execute!(stdout(), DisableLineWrap)
        .map_err(|e| eval.err(format!("term_disable_wrap(): {}", e), span))?;
    Ok(Value::Null)
}
