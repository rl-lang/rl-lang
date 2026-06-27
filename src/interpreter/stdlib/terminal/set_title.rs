use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::stdlib::terminal::common::extract_byte;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{execute, terminal::SetTitle};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "term_set_title", span)?;

    let title = extract_byte(args[0].clone(), "term_set_title", span)?;

    execute!(stdout(), SetTitle(title))
        .map_err(|e| eval.err(format!("term_set_title(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
