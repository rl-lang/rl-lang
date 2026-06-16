use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_set_extension(
    _: &mut Evaluator,
    path: Value,
    target: String,
) -> Result<Value, Error> {
    match path {
        Value::String(s) => {
            let mut buf = std::path::PathBuf::from(&s);
            buf.set_extension(&target);
            Ok(Value::String(buf.to_string_lossy().to_string()))
        }
        other => Err(Error::init(
            format!(
                "path_set_extension() expects a string, got {}",
                other.type_name()
            ),
            None,
            None,
        )),
    }
}
