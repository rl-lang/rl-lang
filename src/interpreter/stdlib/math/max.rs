use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_max(eval: &mut Evaluator, a: Value, b: Value, span: Span) -> Result<Value, Error> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a.max(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(b))),
        (a, b) => Err(eval.err(
            format!(
                "max() expects a number, got ({}, {})",
                a.type_name(),
                b.type_name()
            ),
            span,
        )),
    }
}
