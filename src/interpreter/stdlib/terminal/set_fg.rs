use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::stdlib::terminal::common::extract_byte;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 3, "term_set_fg", span)?;

    let mut iter = args.into_iter();
    let r = extract_byte(iter.next().unwrap(), "r", span)?;
    let g = extract_byte(iter.next().unwrap(), "g", span)?;
    let b = extract_byte(iter.next().unwrap(), "b", span)?;
    execute!(stdout(), SetForegroundColor(Color::Rgb { r, g, b }))
        .map_err(|e| eval.err(format!("term_set_fg(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
