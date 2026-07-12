use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{try_fn, verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{execute, style::ResetColor};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    try_fn!("term_reset_color", execute!(stdout(), ResetColor));
    vok!(vnl!())
}
