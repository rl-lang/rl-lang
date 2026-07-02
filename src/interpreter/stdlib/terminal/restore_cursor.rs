use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{cursor::RestorePosition, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    execute!(stdout(), RestorePosition)
        .map_err(|e| verr!(vs!(format!("term_restore_cursor(): {}", e))));
    vok!(vnl!())
}
