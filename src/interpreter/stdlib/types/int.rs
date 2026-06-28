use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_int(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Integer(_)) || matches!(value, Value::Byte(_))
}

pub fn std_to_int(eval: &mut Evaluator, value: Value, span: Span) -> Result<i64, Error> {
    match value {
        Value::Integer(v) => Ok(v),
        Value::Byte(v) => Ok(v as i64),
        Value::Float(v) => Ok(v as i64),
        Value::Bool(v) => Ok(if v { 1 } else { 0 }),
        Value::Char(v) => Ok(v as i64),
        Value::String(s) => {
            let s = s.trim();
            if s.starts_with("0x") || s.starts_with("0X") {
                i64::from_str_radix(&s[2..], 16)
                    .map_err(|_| eval.err(format!("cannot parse \"{}\" as int", s), span))
            } else {
                s.parse::<i64>()
                    .map_err(|_| eval.err(format!("cannot parse \"{}\" as int", s), span))
            }
        }

        other => Err(eval.err(
            format!("cannot parse \"{}\" as int", other.type_name()),
            span,
        )),
    }
}
