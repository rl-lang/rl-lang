use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_bool(_: &mut Evaluator, value: Value) -> Result<bool, Error> {
    match value {
        Value::Bool(b) => Ok(b),
        Value::Integer(i) => Ok(i != 0),
        Value::Float(f) => Ok(f != 0.0),
        Value::Null => Ok(false),
        Value::String(s) => match s.trim() {
            "true" | "1" => Ok(true),
            "false" | "0" | "" => Ok(false),
            _ => Ok(true),
        },
        Value::Char(_) => Err(Error::init(
            "cannot parse \"char\" as bool".to_string(),
            None,
            None,
        )),
        Value::Function { .. } => Err(Error::init(
            "cannot parse \"function\" as bool".to_string(),
            None,
            None,
        )),
        Value::Values { .. } => Err(Error::init(
            "cannot parse \"array\" as bool".to_string(),
            None,
            None,
        )),
    }
}
