use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{execute, style::Print};
use std::io::stdout;

pub fn func(_: &mut Evaluator, arg: Value) -> Value {
    let text = arg.to_string();

    execute!(stdout(), Print(text)).map_err(|e| return verr!(vs!(format!("term_print(): {}", e))));
    vok!(vnl!())
}
