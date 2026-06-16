use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_exists(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::Bool(std::path::Path::new(s).exists())),
        other => Err(Error::init(
            format!("path_exists() expects a string, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
