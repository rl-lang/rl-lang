use crate::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::{evaluator::Evaluator, values::Value};
use crossterm::cursor::Show;
use crossterm::{
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::io::{Write, stderr, stdout};

pub fn func(_: &mut Evaluator) -> Value {
    let _ = stdout().flush();
    let _ = stderr().flush();
    try_fn!("term_leave", disable_raw_mode());
    try_fn!("term_leave", execute!(stdout(), Show, LeaveAlternateScreen));
    let _ = stdout().flush();

    vok!(vnl!())
}
