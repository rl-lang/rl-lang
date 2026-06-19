use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{
        errors::Error,
        span::Span,
    },
};

pub fn std_rmdir_all(eval: &mut Evaluator, path: String, span: Span) -> Result<Value, Error> {
    std::fs::remove_dir_all(&path).map_err(|e| {
        eval.err(
            format!("rmdir_all(): failed to delete \"{}\": {}", path, e),
            span,
        )
    })?;
    Ok(Value::Null)
}
