use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};
use crossterm::{
    execute,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
};
use std::io::stdout;

pub fn std_term_begin_sync(_: &mut Evaluator) -> Value {
    execute!(stdout(), BeginSynchronizedUpdate)
        .map_err(|e| verr!(vs!(format!("term_begin_sync(): {}", e))));
    vok!(vnl!())
}

pub fn std_term_end_sync(_: &mut Evaluator) -> Value {
    execute!(stdout(), EndSynchronizedUpdate)
        .map_err(|e| verr!(vs!(format!("term_end_sync(): {}", e))));
    vok!(vnl!())
}
