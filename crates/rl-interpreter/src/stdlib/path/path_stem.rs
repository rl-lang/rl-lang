use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_path_stem(eval: &mut Evaluator, path: Value, span: Span) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::Path::new(&s)
                .file_stem()
                .ok_or_else(|| eval.err("file was not found".to_string(), span))?
                .to_string_lossy()
                .to_string(),
        )),
        other => Err(eval.err(
            format!("path_stem() expects a string, got {}", other.type_name()),
            span,
        )),
    }
}
