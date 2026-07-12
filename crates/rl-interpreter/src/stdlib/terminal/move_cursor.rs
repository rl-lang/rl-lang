use crate::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::stdlib::terminal::common::extract_u16;
use crate::{evaluator::Evaluator, values::Value};
use crossterm::{cursor::MoveTo, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator, x: Value, y: Value) -> Value {
    let x = match extract_u16(x, "x") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    let y = match extract_u16(y, "y") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    try_fn!("term_move", execute!(stdout(), MoveTo(x, y)));

    vok!(vnl!())
}
