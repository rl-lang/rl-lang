use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_to_float(_: &mut Evaluator, value: Value) -> Result<f64, Error> {
    match value {
        Value::Float(f) => Ok(f),
        Value::Integer(i) => Ok(i as f64),
        Value::Bool(b) => Ok(if b { 1.0 } else { 0.0 }),
        Value::String(s) => s
            .trim()
            .parse::<f64>()
            .map_err(|_| Error::init(format!("cannot parse \"{}\" as float", s), None, None)),
        Value::Char(_) => Err(Error::init(
            "cannot parse \"char\" as float".to_string(),
            None,
            None,
        )),

        Value::Function { .. } => Err(Error::init(
            "cannot parse \"function\" as float".to_string(),
            None,
            None,
        )),
        Value::Values { .. } => Err(Error::init(
            "cannot parse \"array\" as float".to_string(),
            None,
            None,
        )),
        Value::Null => Err(Error::init(
            "cannot parse \"null\" as float".to_string(),
            None,
            None,
        )),
    }
}
