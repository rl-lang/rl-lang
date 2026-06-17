use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_delete_file(_: &mut Evaluator, file: String) -> Result<Value, Error> {
    std::fs::remove_file(&file).map_err(|e| {
        Error::init(
            format!("read_file(): failed to read \"{}\": {}", file, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::Null)
}
