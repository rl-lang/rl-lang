use tower_lsp::lsp_types::Position;

// since span works with bytes
// this function should simplifi the process
pub fn offset_to_position(source: &str, offset: usize) -> Position {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.chars().filter(|&c| c == '\n').count();
    let character = before.rfind('\n').map(|i| offset - i - 1).unwrap_or(offset);
    Position::new(line as u32, character as u32)
}
