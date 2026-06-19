use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_rmdir(eval: &mut Evaluator, path: String, span: Span) -> Result<Value, Error> {
    std::fs::remove_dir(&path).map_err(|e| {
        eval.err(
            format!("rmdir(): failed to delete \"{}\": {}", path, e),
            span,
        )
    })?;
    Ok(Value::Null)
}
