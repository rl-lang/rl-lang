use crate::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::stdlib::terminal::common::extract_u16;
use crate::{evaluator::Evaluator, values::Value};
use crossterm::{
    execute,
    terminal::{ScrollDown, ScrollUp},
};
use std::io::stdout;

pub fn std_term_scroll_up(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    try_fn!("term_scroll_up", execute!(stdout(), ScrollUp(n)));
    vok!(vnl!())
}

pub fn std_term_scroll_down(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    try_fn!("term_scroll_down", execute!(stdout(), ScrollDown(n)));
    vok!(vnl!())
}
