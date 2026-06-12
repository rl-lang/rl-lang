use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_oct(_: &mut Evaluator, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:o}", v)),
        Value::Float(_) => Err(Error::init(
            "cannot parse \"float\" as octal".to_string(),
            None,
            None,
        )),
        Value::Bool(_) => Err(Error::init(
            "cannot parse \"bool\" as octal".to_string(),
            None,
            None,
        )),
        Value::Char(v) => Ok(format!("{:o}", v as u32)),
        Value::String(s) => Ok(s.bytes().map(|b| format!("{:o}", b)).collect::<String>()),
        Value::Function { .. } => Err(Error::init(
            "cannot parse \"function\" as octal".to_string(),
            None,
            None,
        )),
        Value::Values(_) => Err(Error::init(
            "cannot parse \"array\" as octal".to_string(),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            "cannot parse \"null\" as octal".to_string(),
            None,
            None,
        )),
    }
}
