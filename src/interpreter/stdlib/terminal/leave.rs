use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 0, "term_leave", span)?;

    disable_raw_mode().map_err(|e| eval.err(format!("term_leave(): {}", e), span))?;
    execute!(stdout(), LeaveAlternateScreen)
        .map_err(|e| eval.err(format!("term_leave(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
