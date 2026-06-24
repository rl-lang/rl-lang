//! Conversions between byte offsets and LSP [`Position`]s (line + character).
//!
//! LSP uses 0-indexed `(line, character)` pairs; rl's [`Span`] uses byte offsets
//! into the source string. These two functions bridge that gap.

use tower_lsp::lsp_types::Position;

/// Converts a byte `offset` into the source string into an LSP [`Position`].
///
/// Clamps `offset` to `source.len()` if out of bounds.
pub fn offset_to_position(source: &str, offset: usize) -> Position {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.chars().filter(|&c| c == '\n').count();
    let character = before.rfind('\n').map(|i| offset - i - 1).unwrap_or(offset);
    Position::new(line as u32, character as u32)
}

/// Converts an LSP [`Position`] back into a byte offset into `source`.
///
/// Returns `source.len()` if the position is past the end.
/// The `+1` in the line accumulator accounts for the `\n` separator consumed by `split`.
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
