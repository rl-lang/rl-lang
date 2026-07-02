use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{cursor::Show, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    execute!(stdout(), Show).map_err(|e| return verr!(vs!(format!("term_show_cursor(): {}", e))));
    vok!(vnl!())
}
