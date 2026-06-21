use std::char;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

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
            format!("cannot parse \"{}\" as bool", other.type_name()),
            span,
        )),
    }
}
