use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};
use std::{fs::OpenOptions, io::Write};

pub fn std_append_file(
    eval: &mut Evaluator,
    file: String,
    content: String,
    span: Span,
) -> Result<Value, Error> {
    let mut file_data = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file)
        .map_err(|e| {
            eval.err(
                format!("append_file(): failed to open \"{}\": {}", file, e),
                span,
            )
        })?;

    file_data.write_all(content.as_bytes()).map_err(|e| {
        eval.err(
            format!("append_file(): failed to append \"{}\": {}", file, e),
            span,
        )
    })?;

    Ok(Value::Null)
}
