use crate::interpreter::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::io::stdout;

pub fn std_term_enable_mouse(_: &mut Evaluator) -> Value {
    try_fn!("term_enable_mouse", execute!(stdout(), EnableMouseCapture));
    vok!(vnl!())
}

pub fn std_term_disable_mouse(_: &mut Evaluator) -> Value {
    try_fn!(
        "term_disable_mouse",
        execute!(stdout(), DisableMouseCapture)
    );
    vok!(vnl!())
}
