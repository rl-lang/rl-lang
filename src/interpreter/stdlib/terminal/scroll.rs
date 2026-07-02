use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
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

    execute!(stdout(), ScrollUp(n))
        .map_err(|e| return verr!(vs!(format!("term_scroll_up(): {}", e))));
    vok!(vnl!())
}

pub fn std_term_scroll_down(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    execute!(stdout(), ScrollDown(n))
        .map_err(|e| return verr!(vs!(format!("term_scroll_down(): {}", e))));
    vok!(vnl!())
}
