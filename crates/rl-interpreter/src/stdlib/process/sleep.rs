use crate::{evaluator::Evaluator, stdlib::common::vnl, values::Value};
use std::time::Duration;

pub fn std_sleep(_: &mut Evaluator, ms: i64) -> Value {
    std::thread::sleep(Duration::from_millis(ms.max(0) as u64));
    vnl!()
}
