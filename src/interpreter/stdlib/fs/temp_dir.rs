use std::env::temp_dir;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_temp_dir(_: &mut Evaluator, _: Vec<Value>, _: Span) -> Result<Value, Error> {
    let temp = temp_dir().to_string_lossy().to_string();
    Ok(Value::String(temp))
}
