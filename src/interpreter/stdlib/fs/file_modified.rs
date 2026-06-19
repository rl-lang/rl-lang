use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{
        errors::{Error, ErrorReason, Reason},
        span::Span,
    },
};

pub fn std_file_modified(eval: &mut Evaluator, path: String, span: Span) -> Result<Value, Error> {
    let metadata = std::fs::metadata(&path).map_err(|e| {
        eval.err(
            format!("file_modified(): failed to read \"{}\": {}", path, e),
            span,
        )
    })?;

    let modified = metadata.modified().map_err(|e| {
        eval.err(
            format!(
                "file_modified(): could not get modification time for \"{}\": {}",
                path, e
            ),
            span,
        )
    })?;

    let secs = modified
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| {
            eval.err(
                format!(
                    "file_modified(): modification time before epoch for \"{}\": {}",
                    path, e
                ),
                span,
            )
        })?
        .as_secs();

    Ok(Value::Integer(secs as i64))
}
