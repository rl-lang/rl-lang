use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_log10(eval: &mut Evaluator, a: Value, span: Span) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Float((i as f64).log10())),
        Value::Float(f) => Ok(Value::Float(f.log10())),
        other => Err(eval.err(
            format!("log10() expects a number, got {}", other.type_name()),
            span,
        )),
    }
}
