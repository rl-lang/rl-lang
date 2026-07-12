use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_path_exists(eval: &mut Evaluator, path: Value, span: Span) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::Bool(std::path::Path::new(&s).exists())),
        other => Err(eval.err(
            format!("path_exists() expects a string, got {}", other.type_name()),
            span,
        )),
    }
}
