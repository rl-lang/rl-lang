use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_sqrt(eval: &mut Evaluator, a: Value, span: Span) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Float((i as f64).sqrt())),
        Value::Float(f) => Ok(Value::Float(f.sqrt())),
        other => Err(eval.err(
            format!("round() expects a number, got {}", other.type_name(),),
            span,
        )),
    }
}
