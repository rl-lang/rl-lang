use crate::{interpreter::evaluator::Evaluator, interpreter::values::Value, utils::errors::Error};

pub fn std_len(_: &mut Evaluator, v: Value) -> Result<i64, Error> {
    match v {
        Value::Values { items, .. } => Ok(items.len() as i64),
        Value::String(s) => Ok(s.len() as i64),
        other => Err(Error::init(
            format!(
                "len() expects an array or string, got {}",
                other.type_name()
            ),
            None,
            None,
        )),
    }
}
