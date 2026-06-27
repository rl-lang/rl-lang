use crate::interpreter::stdlib::terminal::common::extract_u16_one_arg;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveToNextLine, MoveToPreviousLine, MoveUp},
    execute,
};
use std::io::stdout;

pub fn std_term_move_up(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    let n = extract_u16_one_arg("term_move_up()", eval, args, span)?;
    execute!(stdout(), MoveUp(n)).map_err(|e| eval.err(format!("term_move_up(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_move_down(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    let n = extract_u16_one_arg("term_move_down()", eval, args, span)?;
    execute!(stdout(), MoveDown(n))
        .map_err(|e| eval.err(format!("term_move_down(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_move_left(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    let n = extract_u16_one_arg("term_move_left()", eval, args, span)?;
    execute!(stdout(), MoveLeft(n))
        .map_err(|e| eval.err(format!("term_move_left(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_move_right(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    let n = extract_u16_one_arg("term_move_right()", eval, args, span)?;
    execute!(stdout(), MoveRight(n))
        .map_err(|e| eval.err(format!("term_move_right(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_next_line(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    let n = extract_u16_one_arg("term_next_line()", eval, args, span)?;
    execute!(stdout(), MoveToNextLine(n))
        .map_err(|e| eval.err(format!("term_next_line(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_prev_line(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    let n = extract_u16_one_arg("term_prev_line()", eval, args, span)?;
    execute!(stdout(), MoveToPreviousLine(n))
        .map_err(|e| eval.err(format!("term_prev_line(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
