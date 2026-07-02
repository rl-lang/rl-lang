use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
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

    execute!(stdout(), MoveTo(x, y)).map_err(|e| return verr!(vs!(format!("term_move(): {}", e))));

    vok!(vnl!())
}
