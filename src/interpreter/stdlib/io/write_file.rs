use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_write_file(
    eval: &mut Evaluator,
    file: String,
    content: String,
    span: Span,
) -> Result<Value, Error> {
    std::fs::write(&file, content).map_err(|e| {
        eval.err(
            format!("write_file(): failed to write \"{}\": {}", file, e),
            span,
        )
    })?;
    Ok(Value::Null)
}
