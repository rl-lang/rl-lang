use crate::{
    evaluator::Evaluator,
    stdlib::common::{vc, verr, vok, vs},
    values::Value,
};

pub fn std_char_at(_: &mut Evaluator, string: String, index: i64) -> Value {
    if index < 0 {
        return verr!(vs!(format!("index cannot be negative: {}", index)));
    }
    let mut chars = string.chars();
    let chars_count = chars.clone().count();
    if index as usize >= chars_count {
        verr!(vs!(format!(
            "index out of bounds string length is {} , used {}",
            chars_count, index
        )))
    } else {
        vok!(vc!(chars.nth(index as usize).unwrap()))
    }
}
