use crate::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::{evaluator::Evaluator, values::Value};
use crossterm::{cursor::Show, execute};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    try_fn!("term_show_cursor", execute!(stdout(), Show));
    vok!(vnl!())
}
