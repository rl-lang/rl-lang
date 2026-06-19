use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_file_size(eval: &mut Evaluator, path: String, span: Span) -> Result<Value, Error> {
    let size = std::fs::metadata(&path)
        .map_err(|e| {
            eval.err(
                format!("file_size(): failed to read \"{}\": {}", path, e),
                span,
            )
        })?
        .len();
    Ok(Value::Integer(size as i64))
}
