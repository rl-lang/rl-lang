use std::sync::Arc;

/// A compact, serializable mapping from byte offsets to 1-indexed
/// `(line, column)` pairs.
///
/// Computed once from source text at compile time. Unlike [`crate::source::SourceFile`],
/// this does not retain the source text itself - only the byte offset
/// where each line starts - so it's cheap enough to embed in compiled
/// `.rlc` bytecode (which intentionally doesn't ship the original
/// source). This lets runtime errors raised from `.rlc` bytecode still
/// report a precise `file:line:col` location instead of a bare message,
/// without paying the cost (or leaking the source) of a full ariadne
/// snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineIndex {
    /// Displayed in error headers (e.g. `"main.rl"`).
    source_name: Arc<str>,
    /// Byte offset of the start of each line; `line_starts[0] == 0`.
    line_starts: Vec<u32>,
}

impl LineIndex {
    /// Builds a [`LineIndex`] by scanning `text` for line breaks.
    pub fn new(source_name: impl Into<Arc<str>>, text: &str) -> Self {
        let mut line_starts = vec![0u32];
        line_starts.extend(
            text.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'\n')
                .map(|(i, _)| (i + 1) as u32),
        );
        Self {
            source_name: source_name.into(),
            line_starts,
        }
    }

    /// Reconstructs a [`LineIndex`] from its raw parts (used when
    /// deserializing from a `.rlc` file).
    pub fn from_raw(source_name: impl Into<Arc<str>>, line_starts: Vec<u32>) -> Self {
        Self {
            source_name: source_name.into(),
            line_starts,
        }
    }

    pub fn source_name(&self) -> &Arc<str> {
        &self.source_name
    }

    pub fn line_starts(&self) -> &[u32] {
        &self.line_starts
    }

    /// 1-indexed `(line, column)` for a byte offset. Clamps out-of-range
    /// offsets to the last known line rather than panicking, since a
    /// slightly-stale span shouldn't crash the error reporter.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let offset = offset as u32;
        let line = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let line_start = self.line_starts.get(line).copied().unwrap_or(0);
        (line + 1, (offset.saturating_sub(line_start)) as usize + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::LineIndex;

    #[test]
    fn line_col_basic() {
        let idx = LineIndex::new("t.rl", "let a = 1;\nlet b = 2;\nprint(a);");
        assert_eq!(idx.line_col(0), (1, 1));
        assert_eq!(idx.line_col(11), (2, 1));
        assert_eq!(idx.line_col(15), (2, 5));
        assert_eq!(idx.line_col(22), (3, 1));
    }
}
