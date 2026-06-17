use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_pop(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => {
            let mut buf = std::path::PathBuf::from(&s);
            buf.pop();
            Ok(Value::String(buf.to_string_lossy().to_string()))
        }
        other => Err(Error::init(
            format!("path_push() expects a string, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
