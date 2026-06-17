use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_file_size(_: &mut Evaluator, path: String) -> Result<Value, Error> {
    let size = std::fs::metadata(&path)
        .map_err(|e| {
            Error::init(
                format!("file_size(): failed to read \"{}\": {}", path, e),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )
        })?
        .len();
    Ok(Value::Integer(size as i64))
}
