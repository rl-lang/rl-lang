use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::io::stdout;

pub fn std_term_enable_mouse(_: &mut Evaluator) -> Value {
    if let Err(e) = execute!(stdout(), EnableMouseCapture) {
        return verr!(vs!(format!("term_enable_mouse(): {}", e)));
    };
    vok!(vnl!())
}

pub fn std_term_disable_mouse(_: &mut Evaluator) -> Value {
    if let Err(e) = execute!(stdout(), DisableMouseCapture) {
        return verr!(vs!(format!("term_disable_mouse(): {}", e)));
    };
    vok!(vnl!())
}
