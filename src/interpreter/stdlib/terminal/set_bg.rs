use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::stdlib::terminal::common::extract_byte;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{
    execute,
    style::{Color, SetBackgroundColor},
};
use std::io::stdout;

pub fn func(_: &mut Evaluator, r: Value, g: Value, b: Value) -> Value {
    let r = match extract_byte(r, "r") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    let b = match extract_byte(b, "b") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    let g = match extract_byte(g, "r") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    execute!(stdout(), SetBackgroundColor(Color::Rgb { r, g, b }))
        .map_err(|e| verr!(vs!(format!("term_set_bg(): {}", e))));
    vok!(vnl!())
}
