use std::env::temp_dir;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_temp_dir(_: &mut Evaluator, _: Vec<Value>) -> Result<Value, Error> {
    let temp = temp_dir().to_string_lossy().to_string();
    Ok(Value::String(temp))
}
