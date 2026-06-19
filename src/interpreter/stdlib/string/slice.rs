use crate::{
    interpreter::evaluator::Evaluator,
    utils::{errors::Error, span::Span},
};

pub fn std_slice(
    eval: &mut Evaluator,
    string: String,
    start: i64,
    end: i64,
    span: Span,
) -> Result<String, Error> {
    let chars = string.chars();
    let chars_count = chars.clone().count();
    if start as usize >= chars_count || end as usize >= chars_count {
        return Err(eval.err(
            format!(
                "index out of bounds string legth: {}, found start: {} and end: {}",
                chars_count, start, end
            ),
            span,
        ));
    }

    Ok(chars
        .skip(start as usize)
        .take(end as usize - start as usize)
        .collect::<String>())
}
