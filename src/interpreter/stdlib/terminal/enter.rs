use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    if let Err(e) = enable_raw_mode() {
        return verr!(vs!(format!("term_enter(): {}", e)));
    }

    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        return verr!(vs!(format!("term_enter(): {}", e)));
    }

    vok!(vnl!())
}
