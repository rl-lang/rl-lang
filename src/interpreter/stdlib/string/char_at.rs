use crate::{
    interpreter::evaluator::Evaluator,
    utils::{errors::Error, span::Span},
};

pub fn std_char_at(
    eval: &mut Evaluator,
    string: String,
    index: i64,
    span: Span,
) -> Result<char, Error> {
    if index < 0 {
        return Err(eval.err(format!("index cannot be negative: {}", index), span));
    }
    let mut chars = string.chars();
    let chars_count = chars.clone().count();
    if index as usize >= chars_count {
        Err(eval.err(
            format!(
                "index out of bounds string length is {} , used {}",
                chars_count, index
            ),
            span,
        ))
    } else {
        Ok(chars.nth(index as usize).unwrap())
    }
}
