use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Range};

use crate::lsp::utils::offset_to_position;

// converts normal Error to diagnostic that lsp can understand
pub fn error_to_diagnostic(source: &str, error: &crate::utils::errors::Error) -> Diagnostic {
    // extract the bytes from the start of span and the end of it
    let (start, end) = error.span().map(|s| (s.start, s.end)).unwrap_or((0, 0));

    // use start and end bytes to make range of the startin col and line to the ending one
    let range = Range::new(
        offset_to_position(source, start),
        offset_to_position(source, end),
    );

    // error severity for errors i guess
    // -- need to add warnings and hints later also information will be nice
    // -- dunno how maybe post v0.2.0 or something
    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        message: error.message().to_string(),
        source: Some("rl".to_string()),
        ..Default::default()
    }
}
