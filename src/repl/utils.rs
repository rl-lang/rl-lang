use crate::repl::lines_types::OutputLine;

pub fn push_error(output: &mut Vec<OutputLine>, e: &crate::utils::errors::Error) {
    output.push(OutputLine::Error(format!("error: {}", e.message())));
}

pub fn char_to_byte(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(b, _)| b)
        .unwrap_or(s.len())
}
