use crate::evaluator::Evaluator;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now(_: &mut Evaluator) -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

pub fn now_ms(_: &mut Evaluator) -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
