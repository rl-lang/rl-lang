use crate::ast::statements::TypeAnnotation;
use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    terminal::{SetSize, size},
};
use std::io::stdout;

pub fn std_term_get_size(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_get_size", span)?;

    let (cols, rows) = size().map_err(|e| eval.err(format!("term_get_size(): {}", e), span))?;
    Ok(Value::Values {
        items_type: TypeAnnotation::Int,
        items: vec![Value::Integer(cols as i64), Value::Integer(rows as i64)],
    })
}

pub fn std_term_set_size(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, "term_set_size", span)?;

    let mut iter = args.into_iter();
    let cols = extract_u16(iter.next().unwrap(), "cols", eval, span)?;
    let rows = extract_u16(iter.next().unwrap(), "rows", eval, span)?;
    execute!(stdout(), SetSize(cols, rows))
        .map_err(|e| eval.err(format!("term_set_size(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
