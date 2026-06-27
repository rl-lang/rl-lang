use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 0, "term_enter", span)?;

    enable_raw_mode().map_err(|e| eval.err(format!("term_enter(): {}", e), span))?;
    execute!(stdout(), EnterAlternateScreen)
        .map_err(|e| eval.err(format!("term_enter(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
