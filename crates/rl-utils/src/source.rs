use std::sync::Arc;

/// A named source file (or `<repl>` snippet) carried through each pipeline
/// stage so error reports can quote the original source text.
#[derive(Clone)]
pub struct SourceFile {
    /// The file name shown in error report headers (e.g. `"main.rl"`, `"<repl>"`).
    pub name: Arc<str>,
    /// The full source text, reference-counted to avoid cloning across pipeline stages.
    pub text: Arc<String>,
}

impl SourceFile {
    /// Creates a new [`SourceFile`] from a name and source text.
    pub fn new(name: impl Into<Arc<str>>, text: impl Into<Arc<String>>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }
}
