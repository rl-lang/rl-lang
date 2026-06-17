use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_hex(_: &mut Evaluator, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:x}", v)),
        Value::Float(_) => Err(Error::init(
            "cannot parse \"float\" as hexadecimal".to_string(),
            None,
            None,
        )),
        Value::Bool(_) => Err(Error::init(
            "cannot parse \"bool\" as hexadecimal".to_string(),
            None,
            None,
        )),
        Value::Char(v) => Ok(format!("{:x}", v as u32)),
        Value::String(s) => Ok(s.bytes().map(|b| format!("{:x}", b)).collect::<String>()),
        Value::Function { .. } => Err(Error::init(
            "cannot parse \"function\" as hexadecimal".to_string(),
            None,
            None,
        )),
        Value::Values { .. } => Err(Error::init(
            "cannot parse \"array\" as hexadecimal".to_string(),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            "cannot parse \"null\" as hexadecimal".to_string(),
            None,
            None,
        )),
    }
}
