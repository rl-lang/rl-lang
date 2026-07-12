use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_path_is_file(eval: &mut Evaluator, path: Value, span: Span) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::Bool(std::path::Path::new(&s).is_file())),
        other => Err(eval.err(
            format!("path_is_file() expects a string, got {}", other.type_name()),
            span,
        )),
    }
}
