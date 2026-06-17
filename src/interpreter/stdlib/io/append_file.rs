use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

use std::{fs::OpenOptions, io::Write};

pub fn std_append_file(_: &mut Evaluator, file: String, content: String) -> Result<Value, Error> {
    let mut file_data = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file)
        .map_err(|e| {
            Error::init(
                format!("append_file(): failed to open \"{}\": {}", file, e),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )
        })?;

    file_data.write_all(content.as_bytes()).map_err(|e| {
        Error::init(
            format!("append_file(): failed to append \"{}\": {}", file, e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;

    Ok(Value::Null)
}
