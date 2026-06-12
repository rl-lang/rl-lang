use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_bin(_: &mut Evaluator, value: Value) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:b}", v)),
        Value::Float(_) => Err(Error::init(
            format!("cannot parse \"float\" as binary"),
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
            format!("cannot parse \"function\" as binary"),
            None,
            None,
        )),
        Value::Values(_) => Err(Error::init(
            format!("cannot parse \"array\" as binary"),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            format!("cannot parse \"null\" as binary"),
            None,
            None,
        )),
    }
}
