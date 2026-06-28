use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_string(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::String(_))
}

pub fn std_to_string(eval: &mut Evaluator, value: Value, span: Span) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{}", v)),
        Value::Byte(v) => Ok(format!("{}", v)),
        Value::Float(v) => Ok(format!("{}", v)),
        Value::Bool(v) => Ok(format!("{}", v)),
        Value::Char(v) => Ok(v.to_string()),
        Value::String(s) => Ok(s),

        other => Err(eval.err(
            format!("cannot parse \"{}\" as string", other.type_name()),
            span,
        )),
    }
}
