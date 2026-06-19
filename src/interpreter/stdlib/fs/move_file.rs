use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{
        errors::{Error, ErrorReason, Reason},
        span::Span,
    },
};

pub fn std_move_file(
    eval: &mut Evaluator,
    src: String,
    dst: String,
    span: Span,
) -> Result<Value, Error> {
    std::fs::rename(&src, &dst).map_err(|e| {
        eval.err(
            format!(
                "move_file(): failed to move \"{}\" to \"{}\": {}",
                src, dst, e
            ),
            span,
        )
    })?;
    Ok(Value::Null)
}
