use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_parent(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::Path::new(&s)
                .parent()
                .ok_or_else(|| Error::init("path was not found".to_string(), None, None))?
                .to_string_lossy()
                .to_string(),
        )),
        other => Err(Error::init(
            format!("path_parent() expects a string, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
