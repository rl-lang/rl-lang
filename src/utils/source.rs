use std::sync::Arc;

/// A named source file (or `<repl>` snippet) carried by each pipeline subsystem
/// so error reports can quote the original code.
#[derive(Clone)]
pub struct SourceFile {
    pub name: Arc<str>,
    pub text: Arc<String>,
}

impl SourceFile {
    pub fn new(name: impl Into<Arc<str>>, text: impl Into<Arc<String>>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }
}
