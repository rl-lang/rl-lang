//! Converts rl [`Error`]s into LSP [`Diagnostic`]s.
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Range};

use crate::utils::offset_to_position;

/// Converts an rl [`Error`] into an LSP [`Diagnostic`].
///
/// Uses the error's primary [`Span`] for the diagnostic range.
/// Falls back to `(0, 0)` if no span is present (legacy errors).
/// All diagnostics are emitted at [`DiagnosticSeverity::ERROR`] -
/// warnings and hints are planned post v0.2.0.
pub fn error_to_diagnostic(source: &str, error: &rl_utils::errors::Error) -> Diagnostic {
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
