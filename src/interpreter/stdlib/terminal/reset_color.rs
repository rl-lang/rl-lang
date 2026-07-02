use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{execute, style::ResetColor};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    execute!(stdout(), ResetColor)
        .map_err(|e| verr!(vs!(format!("term_reset_color(): {}", e))));
    vok!(vnl!())
}
