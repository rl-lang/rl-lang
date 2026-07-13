use crate::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::{evaluator::Evaluator, values::Value};

use crossterm::{
    execute,
    terminal::{DisableLineWrap, EnableLineWrap},
};
use std::io::stdout;

pub fn std_term_enable_wrap(_: &mut Evaluator) -> Value {
    try_fn!("term_enable_wrap", execute!(stdout(), EnableLineWrap));
    vok!(vnl!())
}

pub fn std_term_disable_wrap(_: &mut Evaluator) -> Value {
    try_fn!("term_disable_wrap", execute!(stdout(), DisableLineWrap));
    vok!(vnl!())
}
