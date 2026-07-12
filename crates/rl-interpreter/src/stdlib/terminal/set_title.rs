use crate::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::stdlib::terminal::common::extract_byte;
use crate::{evaluator::Evaluator, values::Value};
use crossterm::{execute, terminal::SetTitle};
use std::io::stdout;

pub fn func(_: &mut Evaluator, arg: Value) -> Value {
    let title = match extract_byte(arg, "term_set_title") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    try_fn!("term_set_title", execute!(stdout(), SetTitle(title)));
    vok!(vnl!())
}
