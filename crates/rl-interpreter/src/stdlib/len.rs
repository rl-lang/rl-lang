use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_len(eval: &mut Evaluator, v: Value, span: Span) -> Result<i64, Error> {
    match v {
        Value::Values { items, .. } => Ok(items.len() as i64),
        Value::String(s) => Ok(s.len() as i64),
        Value::Tuple(items) => Ok(items.len() as i64),
        other => Err(eval.err(
            format!(
                "len() expects an array or string, got {}",
                other.type_name()
            ),
            span,
        )),
    }
}
