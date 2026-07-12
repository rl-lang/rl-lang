use crate::interpreter::stdlib::common::{extract_number, vb, verr, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::event::poll;
use std::time::Duration;

pub fn func(_: &mut Evaluator, arg: Value) -> Value {
    let ms = match extract_number(arg, "ms") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    let ready = match poll(Duration::from_millis(ms)) {
        Ok(v) => v,
        Err(e) => return verr!(vs!(format!("term_poll(): {}", e))),
    };
    vok!(vb!(ready))
}
