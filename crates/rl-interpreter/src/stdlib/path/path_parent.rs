use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_path_parent(eval: &mut Evaluator, path: Value, span: Span) -> Result<Value, Error> {
    match path {
        Value::String(s) => Ok(Value::String(
            std::path::Path::new(&s)
                .parent()
                .ok_or_else(|| eval.err("path was not found".to_string(), span))?
                .to_string_lossy()
                .to_string(),
        )),
        other => Err(eval.err(
            format!("path_parent() expects a string, got {}", other.type_name()),
            span,
        )),
    }
}
