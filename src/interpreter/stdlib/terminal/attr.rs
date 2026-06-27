use crate::interpreter::stdlib::common::check_arity;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
use crossterm::{
    execute,
    style::{Attribute, SetAttribute},
};
use std::io::stdout;

fn set_attr(
    attr: Attribute,
    name: &str,
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 0, name, span)?;

    execute!(stdout(), SetAttribute(attr))
        .map_err(|e| eval.err(format!("{}(): {}", name, e), span))?;
    Ok(Value::Null)
}

pub fn std_term_bold(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    set_attr(Attribute::Bold, "term_bold", eval, args, span)
}

pub fn std_term_dim(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    set_attr(Attribute::Dim, "term_dim", eval, args, span)
}

pub fn std_term_italic(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    set_attr(Attribute::Italic, "term_italic", eval, args, span)
}

pub fn std_term_underline(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    set_attr(Attribute::Underlined, "term_underline", eval, args, span)
}

pub fn std_term_blink(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    set_attr(Attribute::SlowBlink, "term_blink", eval, args, span)
}

pub fn std_term_reverse(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    set_attr(Attribute::Reverse, "term_reverse", eval, args, span)
}

pub fn std_term_crossed_out(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    set_attr(Attribute::CrossedOut, "term_crossed_out", eval, args, span)
}

pub fn std_term_reset_attr(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    set_attr(Attribute::Reset, "term_reset_attr", eval, args, span)
}
