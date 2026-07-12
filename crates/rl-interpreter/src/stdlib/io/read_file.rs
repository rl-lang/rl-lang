use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_read_file(eval: &mut Evaluator, file: String, span: Span) -> Result<Value, Error> {
    let data = std::fs::read_to_string(&file).map_err(|e| {
        eval.err(
            format!("read_file(): failed to read \"{}\": {}", file, e),
            span,
        )
    })?;
    Ok(Value::String(data))
}
