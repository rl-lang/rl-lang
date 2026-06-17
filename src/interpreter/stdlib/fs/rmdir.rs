use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_rmdir(_: &mut Evaluator, path: String) -> Result<Value, Error> {
    std::fs::remove_dir(&path).map_err(|e| {
        Error::init(
            format!("rmdir(): failed to delete \"{}\": {}", path, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::Null)
}
