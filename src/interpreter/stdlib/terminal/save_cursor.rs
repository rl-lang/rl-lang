use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{cursor::SavePosition, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    execute!(stdout(), SavePosition)
        .map_err(|e| return verr!(vs!(format!("term_save_cursor(): {}", e))));
    vok!(vnl!())
}
