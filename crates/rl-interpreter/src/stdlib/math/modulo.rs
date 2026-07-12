use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_mod(eval: &mut Evaluator, a: Value, b: Value, span: Span) -> Result<Value, Error> {
    let a = normalize_numeric(a);
    let b = normalize_numeric(b);

    match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(a as f64 % b)),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a % b as f64)),
        (a, b) => Err(eval.err(
            format!(
                "mod() expects a number, got ({}, {})",
                a.type_name(),
                b.type_name()
            ),
            span,
        )),
    }
}

fn normalize_numeric(v: Value) -> Value {
    match v {
        Value::Byte(b) => Value::Integer(b as i64),
        other => other,
    }
}
