use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{try_fn, verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    try_fn!("term_enter", enable_raw_mode());
    try_fn!("term_enter", execute!(stdout(), EnterAlternateScreen));

    vok!(vnl!())
}
