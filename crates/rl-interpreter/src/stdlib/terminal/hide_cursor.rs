use crate::{
    evaluator::Evaluator,
    stdlib::common::{try_fn, verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{cursor::Hide, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    try_fn!("term_hide_cursor", execute!(stdout(), Hide));

    vok!(vnl!())
}
