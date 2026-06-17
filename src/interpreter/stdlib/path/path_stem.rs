use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_stem(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::Path::new(&s)
                .file_stem()
                .ok_or_else(|| Error::init("file was not found".to_string(), None, None))?
                .to_string_lossy()
                .to_string(),
        )),
        other => Err(Error::init(
            format!("path_stem() expects a string, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
