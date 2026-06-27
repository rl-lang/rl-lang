use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    terminal::{ScrollDown, ScrollUp},
};
use std::io::stdout;

pub fn std_term_scroll_up(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 1, "term_scroll_down", span)?;

    let n = extract_u16(args.into_iter().next().unwrap(), "n", eval, span)?;
    execute!(stdout(), ScrollUp(n))
        .map_err(|e| eval.err(format!("term_scroll_up(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_scroll_down(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 1, "term_scroll_down", span)?;

    let n = extract_u16(args.into_iter().next().unwrap(), "n", eval, span)?;
    execute!(stdout(), ScrollDown(n))
        .map_err(|e| eval.err(format!("term_scroll_down(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
