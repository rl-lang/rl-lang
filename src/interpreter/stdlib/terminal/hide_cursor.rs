use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{cursor::Hide, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    execute!(stdout(), Hide).map_err(|e| return verr!(vs!(format!("term_hide_cursor(): {}", e))));

    vok!(vnl!())
}
