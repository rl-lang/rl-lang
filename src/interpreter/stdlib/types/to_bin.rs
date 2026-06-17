use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_bin(_: &mut Evaluator, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:b}", v)),
        Value::Float(_) => Err(Error::init(
            "cannot parse \"float\" as binary".to_string(),
            None,
            None,
        )),
        Value::Bool(v) => {
            if v {
                Ok("1".to_string())
            } else {
                Ok("0".to_string())
            }
        }
        Value::Char(v) => Ok(format!("{:b}", v as u32)),
        Value::String(s) => Ok(s.bytes().map(|b| format!("{:b}", b)).collect::<String>()),
        Value::Function { .. } => Err(Error::init(
            "cannot parse \"function\" as binary".to_string(),
            None,
            None,
        )),
        Value::Values { .. } => Err(Error::init(
            "cannot parse \"array\" as binary".to_string(),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            "cannot parse \"null\" as binary".to_string(),
            None,
            None,
        )),
    }
}
