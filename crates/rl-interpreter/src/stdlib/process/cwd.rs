use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_cwd(eval: &mut Evaluator, _: Vec<Value>, span: Span) -> Result<Value, Error> {
    std::env::current_dir()
        .map(|p| Value::String(p.to_string_lossy().to_string()))
        .map_err(|e| eval.err(format!("cwd(): {}", e), span))
}

pub fn std_set_cwd(eval: &mut Evaluator, path: String, span: Span) -> Result<Value, Error> {
    std::env::set_current_dir(&path)
        .map(|_| Value::Null)
        .map_err(|e| {
            eval.err(
                format!("set_cwd(): failed to change to \"{}\": {}", path, e),
                span,
            )
        })
}
