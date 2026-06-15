use crate::{
    interpreter::evaluator::Evaluator,
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_char_at(_: &mut Evaluator, string: String, index: i64) -> Result<char, Error> {
    if index < 0 {
        return Err(Error::init(
            format!("index cannot be negative: {}", index),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        ));
    }
    let mut chars = string.chars();
    let chars_count = chars.clone().count();
    if index as usize >= chars_count {
        Err(Error::init(
            format!(
                "index out of bounds string length is {} , used {}",
                chars_count, index
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        ))
    } else {
        Ok(chars.nth(index as usize).unwrap())
    }
}
