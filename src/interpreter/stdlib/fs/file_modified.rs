use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_file_modified(_: &mut Evaluator, path: String) -> Result<Value, Error> {
    let metadata = std::fs::metadata(&path).map_err(|e| {
        Error::init(
            format!("file_modified(): failed to read \"{}\": {}", path, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;

    let modified = metadata.modified().map_err(|e| {
        Error::init(
            format!(
                "file_modified(): could not get modification time for \"{}\": {}",
                path, e
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;

    let secs = modified
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| {
            Error::init(
                format!(
                    "file_modified(): modification time before epoch for \"{}\": {}",
                    path, e
                ),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )
        })?
        .as_secs();

    Ok(Value::Integer(secs as i64))
}
