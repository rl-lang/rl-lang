use tower_lsp::lsp_types::Position;

// since span works with bytes
// this function should simplify the process
pub fn offset_to_position(source: &str, offset: usize) -> Position {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.chars().filter(|&c| c == '\n').count();
    let character = before.rfind('\n').map(|i| offset - i - 1).unwrap_or(offset);
    Position::new(line as u32, character as u32)
}

// inverse of offset_to_position
pub fn position_to_offset(source: &str, position: Position) -> usize {
    let mut offset = 0usize;
    for (i, line) in source.split('\n').enumerate() {
        if i as u32 == position.line {
            let char_offset = (position.character as usize).min(line.len());
            return offset + char_offset;
        }
        // +1 to skip over the '\n' itself
        offset += line.len() + 1;
    }
    source.len()
}
