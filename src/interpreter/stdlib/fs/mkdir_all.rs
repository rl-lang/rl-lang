use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_mkdir_all(_: &mut Evaluator, path: String) -> Result<Value, Error> {
    std::fs::create_dir_all(&path).map_err(|e| {
        Error::init(
            format!("mkdir_all(): failed to create \"{}\": {}", path, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::Null)
}
