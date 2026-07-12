use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_delete_file(eval: &mut Evaluator, file: String, span: Span) -> Result<Value, Error> {
    std::fs::remove_file(&file).map_err(|e| {
        eval.err(
            format!("delete_file(): failed to read \"{}\": {}", file, e),
            span,
        )
    })?;
    Ok(Value::Null)
}
