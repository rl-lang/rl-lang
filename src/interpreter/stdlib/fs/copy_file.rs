use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_copy_file(_: &mut Evaluator, src: String, dst: String) -> Result<Value, Error> {
    let bytes = std::fs::copy(&src, &dst).map_err(|e| {
        Error::init(
            format!(
                "copy_file(): failed to copy \"{}\" to \"{}\": {}",
                src, dst, e
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::Integer(bytes as i64))
}
