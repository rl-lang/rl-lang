use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_min(eval: &mut Evaluator, a: Value, b: Value, span: Span) -> Result<Value, Error> {
    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a.min(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(b))),
        (a, b) => Err(eval.err(
            format!(
                "min() expects a number, got ({}, {})",
                a.type_name(),
                b.type_name()
            ),
            span,
        )),
    }
}
