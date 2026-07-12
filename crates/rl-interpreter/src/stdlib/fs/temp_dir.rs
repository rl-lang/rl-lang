use std::env::temp_dir;

use crate::{evaluator::Evaluator, stdlib::common::vs, values::Value};

pub fn std_temp_dir(_: &mut Evaluator) -> Value {
    vs!(temp_dir().to_string_lossy().to_string())
}
