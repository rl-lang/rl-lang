use crate::{
    evaluator::Evaluator,
    stdlib::common::{try_fn, verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{cursor::RestorePosition, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    try_fn!("term_restore_cursor", execute!(stdout(), RestorePosition));
    vok!(vnl!())
}
