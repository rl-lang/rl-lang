//! Output line variants used by the REPL output buffer.
use ratatui::style::Style;

/// A line in the REPL output buffer.
pub enum OutputLine {
    /// Raw user input - rendered with `>>` or `..` prompt and syntax highlighting.
    Input(String),
    /// Successfully evaluated input - stored for `:save` but not rendered.
    ValidInput(String),
    /// Output produced by evaluating an expression (e.g. a value or `println` output).
    Result(String),
    /// An error message - rendered in red with a `✗` prefix.
    Error(String),
    /// An informational message - rendered in dark gray (used by commands and startup).
    Info(String),
    /// A pre-styled run of `(text, Style)` pairs - used by `:help` and `:stdlib`.
    Styled(Vec<(String, Style)>),
    /// A horizontal separator line.
    Separator,
}
