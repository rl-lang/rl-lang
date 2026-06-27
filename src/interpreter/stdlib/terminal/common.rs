use crate::{
    interpreter::{evaluator::Evaluator, stdlib::common::check_arity, values::Value},
    utils::{errors::Error, span::Span},
};


pub fn extract_u16(v: Value, name: &str, eval: &mut Evaluator, span: Span) -> Result<u16, Error> {
    match v {
        Value::Integer(i) if i >= 0 => Ok(i as u16),
        Value::Integer(i) => Err(eval.err(format!("{} must be >= 0, got {}", name, i), span)),
        other => Err(eval.err(
            format!("{} must be int, got {}", name, other.type_name()),
            span,
        )),
    }
}
