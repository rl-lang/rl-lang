use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};

use crossterm::{
    execute,
    terminal::{DisableLineWrap, EnableLineWrap},
};
use std::io::stdout;

pub fn std_term_enable_wrap(_: &mut Evaluator) -> Value {
    execute!(stdout(), EnableLineWrap)
        .map_err(|e| return verr!(vs!(format!("term_enable_wrap(): {}", e))));
    vok!(vnl!())
}

pub fn std_term_disable_wrap(_: &mut Evaluator) -> Value {
    execute!(stdout(), DisableLineWrap)
        .map_err(|e| return verr!(vs!(format!("term_disable_wrap(): {}", e))));
    vok!(vnl!())
}
