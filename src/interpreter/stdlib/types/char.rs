use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_is_char(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Char(_))
}

pub fn std_to_char(eval: &mut Evaluator, value: Value, span: Span) -> Result<char, Error> {
    match value {
        Value::Char(c) => Ok(c),
        Value::Integer(i) => char::from_u32(i as u32)
            .ok_or_else(|| eval.err(format!("{} is not a valid unicode codepoint", i), span)),
        Value::Byte(i) => char::from_u32(i as u32)
            .ok_or_else(|| eval.err(format!("{} is not a valid unicode codepoint", i), span)),
        Value::String(s) => {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(eval.err("string must be exactly one character".to_string(), span)),
            }
        }

        other => Err(eval.err(
            format!("cannot parse \"{}\" as character", other.type_name()),
            span,
        )),
    }
}
