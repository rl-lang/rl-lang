use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::stdlib::terminal::common::extract_byte;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{execute, terminal::SetTitle};
use std::io::stdout;

pub fn func(_: &mut Evaluator, arg: Value) -> Value {
    let title = match extract_byte(arg, "term_set_title") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    execute!(stdout(), SetTitle(title))
        .map_err(|e| return verr!(vs!(format!("term_set_title(): {}", e))));
    vok!(vnl!())
}
