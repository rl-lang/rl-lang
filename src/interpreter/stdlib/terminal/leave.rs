use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::cursor::Show;
use crossterm::{
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::io::{Write, stderr, stdout};

pub fn func(_: &mut Evaluator) -> Value {
    let _ = stdout().flush();
    let _ = stderr().flush();
    if let Err(e) = disable_raw_mode() {
        return verr!(vs!(format!("term_leave(): {}", e)));
    };

    if let Err(e) = execute!(stdout(), Show, LeaveAlternateScreen) {
        return verr!(vs!(format!("term_leave(): {}", e)));
    };
    let _ = stdout().flush();

    vok!(vnl!())
}
