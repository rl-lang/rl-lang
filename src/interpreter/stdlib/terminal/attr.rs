use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{
    execute,
    style::{Attribute, SetAttribute},
};
use std::io::stdout;

fn set_attr(attr: Attribute, name: &str) -> Value {
    match execute!(stdout(), SetAttribute(attr)) {
        Err(e) => verr!(vs!(format!("{}(): {}", name, e))),
        Ok(_) => vok!(vnl!()),
    }
}

pub fn std_term_bold(_: &mut Evaluator) -> Value {
    set_attr(Attribute::Bold, "term_bold")
}

pub fn std_term_dim(_: &mut Evaluator) -> Value {
    set_attr(Attribute::Dim, "term_dim")
}

pub fn std_term_italic(_: &mut Evaluator) -> Value {
    set_attr(Attribute::Italic, "term_italic")
}

pub fn std_term_underline(_: &mut Evaluator) -> Value {
    set_attr(Attribute::Underlined, "term_underline")
}

pub fn std_term_blink(_: &mut Evaluator) -> Value {
    set_attr(Attribute::SlowBlink, "term_blink")
}

pub fn std_term_reverse(_: &mut Evaluator) -> Value {
    set_attr(Attribute::Reverse, "term_reverse")
}

pub fn std_term_crossed_out(_: &mut Evaluator) -> Value {
    set_attr(Attribute::CrossedOut, "term_crossed_out")
}

pub fn std_term_reset_attr(_: &mut Evaluator) -> Value {
    set_attr(Attribute::Reset, "term_reset_attr")
}
