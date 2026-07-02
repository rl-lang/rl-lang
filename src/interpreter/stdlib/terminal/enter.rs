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
    enable_raw_mode().map_err(|e| return verr!(vs!(format!("term_enter(): {}", e))));

    execute!(stdout(), EnterAlternateScreen)
        .map_err(|e| return verr!(vs!(format!("term_enter(): {}", e))));

    vok!(vnl!())
}
