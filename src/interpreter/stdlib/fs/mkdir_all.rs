use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_mkdir_all(eval: &mut Evaluator, path: String, span: Span) -> Result<Value, Error> {
    std::fs::create_dir_all(&path).map_err(|e| {
        eval.err(
            format!("mkdir_all(): failed to create \"{}\": {}", path, e),
            span,
        )
    })?;
    Ok(Value::Null)
}
