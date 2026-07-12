use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{try_fn, verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{execute, style::Print};
use std::io::stdout;

pub fn func(_: &mut Evaluator, arg: Value) -> Value {
    let text = arg.to_string();

    try_fn!("term_print", execute!(stdout(), Print(text)));
    vok!(vnl!())
}
