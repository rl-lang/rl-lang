use crate::interpreter::stdlib::common::{verr, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn func(_: &mut Evaluator) -> Value {
    match execute!(stdout(), Clear(ClearType::All)) {
        Err(e) => verr!(vs!(format!("term_clear(): {}", e))),
        Ok(_) => Value::Ok(Box::new(Value::Null)),
    }
}
