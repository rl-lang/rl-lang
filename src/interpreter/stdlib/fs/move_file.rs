use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_move_file(_: &mut Evaluator, src: String, dst: String) -> Result<Value, Error> {
    std::fs::rename(&src, &dst).map_err(|e| {
        Error::init(
            format!(
                "move_file(): failed to move \"{}\" to \"{}\": {}",
                src, dst, e
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::Null)
}
