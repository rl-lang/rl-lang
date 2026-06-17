use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_write_file(_: &mut Evaluator, file: String, content: String) -> Result<Value, Error> {
    std::fs::write(&file, content).map_err(|e| {
        Error::init(
            format!("write_file(): failed to write \"{}\": {}", file, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::Null)
}
