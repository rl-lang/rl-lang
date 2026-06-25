use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};
use std::time::Duration;

pub fn std_sleep(_: &mut Evaluator, ms: i64, _: Span) -> Result<Value, Error> {
    std::thread::sleep(Duration::from_millis(ms.max(0) as u64));
    Ok(Value::Null)
}
