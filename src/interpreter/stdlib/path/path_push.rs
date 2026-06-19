use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_path_push(
    eval: &mut Evaluator,
    path: Value,
    target: String,
    span: Span,
) -> Result<Value, Error> {
    match path {
        Value::String(s) => {
            let mut buf = std::path::PathBuf::from(&s);
            buf.push(&target);
            Ok(Value::String(buf.to_string_lossy().to_string()))
        }
        other => Err(eval.err(
            format!("path_push() expects a string, got {}", other.type_name()),
            span,
        )),
    }
}
