use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_float(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Float(_))
}

pub fn std_to_float(eval: &mut Evaluator, value: Value, span: Span) -> Result<f64, Error> {
    match value {
        Value::Float(f) => Ok(f),
        Value::Integer(i) => Ok(i as f64),
        Value::Byte(i) => Ok(i as f64),
        Value::Bool(b) => Ok(if b { 1.0 } else { 0.0 }),
        Value::String(s) => s
            .trim()
            .parse::<f64>()
            .map_err(|_| eval.err(format!("cannot parse \"{}\" as float", s), span)),

        other => Err(eval.err(
            format!("cannot parse \"{}\" as float", other.type_name()),
            span,
        )),
    }
}
