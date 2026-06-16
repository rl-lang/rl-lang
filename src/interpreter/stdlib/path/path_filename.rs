use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_filename(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::Path::new(s).file_name().to_string(),
        )),
        other => Err(Error::init(
            format!(
                "path_filename() expects a string, got {}",
                other.type_name()
            ),
            None,
            None,
        )),
    }
}
