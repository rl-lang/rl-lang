use crate::{interpreter::evaluator::Evaluator, utils::errors::Error};

pub fn std_slice(_: &mut Evaluator, string: String, start: i64, end: i64) -> Result<String, Error> {
    let chars = string.chars();
    let chars_count = chars.clone().count();
    if start as usize >= chars_count || end as usize >= chars_count {
        return Err(Error::init(
            format!(
                "index out of bounds string legth: {}, found start: {} and end: {}",
                chars_count, start, end
            ),
            None,
            None,
        ));
    }

    Ok(chars
        .skip(start as usize)
        .take(end as usize - start as usize)
        .collect::<String>())
}
