use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_string(_: &mut Evaluator, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{}", v)),
        Value::Float(v) => Ok(format!("{}", v)),
        Value::Bool(v) => Ok(format!("{}", v)),
        Value::Char(v) => Ok(v.to_string()),
        Value::String(s) => Ok(s),
        Value::Function { .. } => Err(Error::init(
            "cannot parse \"function\" as string".to_string(),
            None,
            None,
        )),
        Value::Values { .. } => Err(Error::init(
            "cannot parse \"array\" as string".to_string(),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            "cannot parse \"null\" as string".to_string(),
            None,
            None,
        )),
    }
}
