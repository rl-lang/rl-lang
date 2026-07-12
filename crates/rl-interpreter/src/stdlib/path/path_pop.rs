use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_path_pop(eval: &mut Evaluator, path: Value, span: Span) -> Result<Value, Error> {
    match path {
        Value::String(s) => {
            let mut buf = std::path::PathBuf::from(&s);
            buf.pop();
            Ok(Value::String(buf.to_string_lossy().to_string()))
        }
        other => Err(eval.err(
            format!("path_push() expects a string, got {}", other.type_name()),
            span,
        )),
    }
}
