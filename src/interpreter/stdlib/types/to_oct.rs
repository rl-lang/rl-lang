use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_oct(_: &mut Evaluator, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:o}", v)),
        Value::Float(_) => Err(Error::init(
            format!("cannot parse \"float\" as octal"),
            None,
            None,
        )),
        Value::Bool(_) => Err(Error::init(
            format!("cannot parse \"bool\" as octal"),
            None,
            None,
        )),
        Value::Char(v) => Ok(format!("{:o}", v as u32)),
        Value::String(s) => Ok(s.bytes().map(|b| format!("{:o}", b)).collect::<String>()),
        Value::Function { .. } => Err(Error::init(
            format!("cannot parse \"function\" as octal"),
            None,
            None,
        )),
        Value::Values(_) => Err(Error::init(
            format!("cannot parse \"array\" as octal"),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            format!("cannot parse \"null\" as octal"),
            None,
            None,
        )),
    }
}
