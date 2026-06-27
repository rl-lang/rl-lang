use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{cursor::MoveTo, execute};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 2, "term_move", span)?;

    let mut iter = args.into_iter();
    let x = extract_u16(iter.next().unwrap(), "x", eval, span)?;
    let y = extract_u16(iter.next().unwrap(), "y", eval, span)?;
    execute!(stdout(), MoveTo(x, y)).map_err(|e| eval.err(format!("term_move(): {}", e), span))?;

    Ok(Value::Ok(Box::new(Value::Null)))
}
