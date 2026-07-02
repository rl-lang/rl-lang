use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{try_fn, verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{cursor::SavePosition, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    try_fn!("term_save_cursor", execute!(stdout(), SavePosition));
    vok!(vnl!())
}
