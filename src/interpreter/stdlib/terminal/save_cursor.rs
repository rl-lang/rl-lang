use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{cursor::SavePosition, execute};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 0, "term_save_cursor", span)?;

    execute!(stdout(), SavePosition)
        .map_err(|e| eval.err(format!("term_save_cursor(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
