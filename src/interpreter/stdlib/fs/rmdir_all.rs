use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_rmdir_all(_: &mut Evaluator, path: String) -> Result<Value, Error> {
    std::fs::remove_dir_all(&path).map_err(|e| {
        Error::init(
            format!("rmdir_all(): failed to delete \"{}\": {}", path, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::Null)
}
