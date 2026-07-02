use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::stdlib::terminal::common::extract_u16;
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{cursor::MoveToRow, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator, args: Value) -> Value {
    let col = match extract_u16(args, "row") {
        Ok(a) => a,
        Err(e) => return verr!(vs!(e)),
    };
    match execute!(stdout(), MoveToRow(col)) {
        Err(e) => verr!(vs!(format!("term_move_to_row(): {}", e))),
        Ok(_) => vok!(vnl!()),
    }
}
