use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{cursor::Hide, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    if let Err(e) = execute!(stdout(), Hide) {
        return verr!(vs!(format!("term_hide_cursor(): {}", e)));
    };

    vok!(vnl!())
}
