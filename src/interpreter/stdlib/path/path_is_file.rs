use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_is_file(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::Bool(std::path::Path::new(&s).is_file())),
        other => Err(Error::init(
            format!("path_is_file() expects a string, got {}", other.type_name()),
            None,
            None,
        )),
    }
}
