use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_is_dir(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::Bool(std::path::Path::new(&s).is_dir())),
        other => Err(Error::init(
            format!("path_is_dir() expects a string, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
