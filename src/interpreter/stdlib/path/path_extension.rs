use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_path_extension(_: &mut Evaluator, path: Value) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::Path::new(s).extension().to_string(),
        )),
        other => Err(Error::init(
            format!(
                "path_extension() expects a string, got {}",
                other.type_name()
            ),
            None,
            None,
        )),
    }
}
