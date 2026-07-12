use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_pow(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.err(
            format!("pow() expects 2 arguments, got {}", args.len()),
            span,
        ));
    }
    let mut iter = args.into_iter();
    let base = normalize_numeric(iter.next().unwrap());
    let exponent = normalize_numeric(iter.next().unwrap());

    match (base, exponent) {
        (Value::Integer(a), Value::Integer(b)) => {
            let b = b as u32;
            Ok(Value::Integer(a.pow(b)))
        }
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64).powf(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powi(b as i32))),
        _ => Err(eval.err("pow() expects numeric arguments".to_string(), span)),
    }
}

// widen the byte to integer
// more types will be implemented later
// or being strict after adding explicit cast
fn normalize_numeric(v: Value) -> Value {
    match v {
        Value::Byte(b) => Value::Integer(b as i64),
        other => other,
    }
}
