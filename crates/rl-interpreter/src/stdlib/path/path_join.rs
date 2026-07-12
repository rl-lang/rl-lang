use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_path_join(
    eval: &mut Evaluator,
    path: Value,
    target: String,
    span: Span,
) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::PathBuf::from(&s)
                .join(&target)
                .to_string_lossy()
                .to_string(),
        )),
        other => Err(eval.err(
            format!("path_join() expects a string, got {}", other.type_name()),
            span,
        )),
    }
}
