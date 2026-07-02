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
    disable_raw_mode().map_err(|e| verr!(vs!(format!("term_leave(): {}", e))));

    execute!(stdout(), Show, LeaveAlternateScreen)
        .map_err(|e| verr!(vs!(format!("term_leave(): {}", e))));
    let _ = stdout().flush();

    vok!(vnl!())
}
