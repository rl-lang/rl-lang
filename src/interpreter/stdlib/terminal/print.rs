use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{execute, style::Print};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "term_print", span)?;

    let text = args.into_iter().next().unwrap().to_string();
    execute!(stdout(), Print(text)).map_err(|e| eval.err(format!("term_print(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
