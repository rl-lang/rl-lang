use std::char;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_char(_: &mut Evaluator, value: Value) -> Result<char, Error> {
    match value {
        Value::Char(c) => Ok(c),
        Value::Integer(i) => char::from_u32(i as u32).ok_or_else(|| {
            Error::init(
                format!("{} is not a valid unicode codepoint", i),
                None,
                None,
            )
        }),
        Value::String(s) => {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => Ok(c),
                _ => Err(Error::init(
                    "string must be exactly one character".to_string(),
                    None,
                    None,
                )),
            }
        }

        Value::Float(_) => Err(Error::init(
            format!("cannot parse \"float\" as character"),
            None,
            None,
        )),
        Value::Bool(_) => Err(Error::init(
            format!("cannot parse \"bool\" as character"),
            None,
            None,
        )),

        Value::Function { .. } => Err(Error::init(
            format!("cannot parse \"function\" as character"),
            None,
            None,
        )),
        Value::Values(_) => Err(Error::init(
            format!("cannot parse \"array\" as character"),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            format!("cannot parse \"null\" as character"),
            None,
            None,
        )),
    }
}
