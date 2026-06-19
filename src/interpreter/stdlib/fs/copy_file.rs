use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{
        errors::{Error, ErrorReason, Reason},
        span::Span,
    },
};

pub fn std_copy_file(
    eval: &mut Evaluator,
    src: String,
    dst: String,
    span: Span,
) -> Result<Value, Error> {
    let bytes = std::fs::copy(&src, &dst).map_err(|e| {
        eval.err(
            format!(
                "copy_file(): failed to copy \"{}\" to \"{}\": {}",
                src, dst, e
            ),
            span,
        )
    })?;
    Ok(Value::Integer(bytes as i64))
}
