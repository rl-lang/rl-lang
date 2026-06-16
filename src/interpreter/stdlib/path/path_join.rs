use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_join(_: &mut Evaluator, path: Value, target: String) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::PathBuf::from(&s)
                .join(&target)
                .to_string_lossy()
                .to_string(),
        )),
        other => Err(Error::init(
            format!("path_join() expects a string, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
