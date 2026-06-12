use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_int(_: &mut Evaluator, value: Value) -> Result<i64, Error> {
    match value {
        Value::Integer(v) => Ok(v),
        Value::Float(v) => Ok(v as i64),
        Value::Bool(v) => Ok(if v { 1 } else { 0 }),
        Value::Char(v) => Ok(v as i64),
        Value::String(s) => {
            let s = s.trim();
            if s.starts_with("0x") || s.starts_with("0X") {
                i64::from_str_radix(&s[2..], 16)
                    .map_err(|_| Error::init(format!("cannot parse \"{}\" as int", s), None, None))
            } else {
                s.parse::<i64>()
                    .map_err(|_| Error::init(format!("cannot parse \"{}\" as int", s), None, None))
            }
        }
        Value::Function { .. } => Err(Error::init(
            "cannot parse \"function\" as int".to_string(),
            None,
            None,
        )),
        Value::Values(_) => Err(Error::init(
            "cannot parse \"array\" as int".to_string(),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            "cannot parse \"null\" as int".to_string(),
            None,
            None,
        )),
    }
}
