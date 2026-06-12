use crate::{interpreter::evaluator::Evaluator, utils::errors::Error};

pub fn std_char_at(_: &mut Evaluator, string: String, index: i64) -> Result<char, Error> {
    let mut chars = string.chars();
    let chars_count = chars.clone().count();
    if index as usize >= chars_count {
        Err(Error::init(
            format!(
                "index out of bounds string length is {} , used {}",
                chars_count, index
            ),
            None,
            None,
        ))
    } else {
        Ok(chars.nth(index as usize).unwrap())
    }
}
