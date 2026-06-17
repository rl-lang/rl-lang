use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_read_file(_: &mut Evaluator, file: String) -> Result<Value, Error> {
    let data = std::fs::read_to_string(&file).map_err(|e| {
        Error::init(
            format!("read_file(): failed to read \"{}\": {}", file, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::String(data))
}
