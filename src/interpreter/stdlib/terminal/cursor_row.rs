use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{cursor::MoveToRow, execute};
use std::io::stdout;

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "term_move_to_row", span)?;

    let row = extract_u16(args.into_iter().next().unwrap(), "row", eval, span)?;
    execute!(stdout(), MoveToRow(row))
        .map_err(|e| eval.err(format!("term_move_to_row(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
