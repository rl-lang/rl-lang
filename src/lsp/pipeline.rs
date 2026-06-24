//! Diagnostic pipeline: lex -> parse -> type-check.
//!
//! The evaluator is intentionally excluded. Running user code on every
//! keystroke caused the LSP to hang on infinite loops (e.g. `while true {}`).
//! The [`TypeChecker`] walks the AST without executing anything, making it
//! always safe to run on in-progress or non-terminating source.
use crate::{
    checker::TypeChecker, lexer::tokenizer::Tokenizer, lsp::to_diagnostic::error_to_diagnostic,
    parser::parser_logic::Parser, utils::source::SourceFile,
};
use tower_lsp::lsp_types::Diagnostic;

/// lex -> parse -> type-check the given source string and return LSP diagnostics.
///
/// the evaluator is no longer called here: running user code on every keystroke
/// caused the LSP to hang on infinite loops (e.g. `while true {}`). The
/// [`TypeChecker`] walks the same AST without executing anything, so it is
/// always safe to run on in-progress or even non-terminating source.
pub fn run_pipeline(source: &str) -> Vec<Diagnostic> {
    let file = SourceFile::new("buffer", source.to_string());

    let tokens = match Tokenizer::lex(file.clone()) {
        Ok(t) => t,
        Err(e) => return vec![error_to_diagnostic(source, &e)],
    };

    let statements = match Parser::parse(tokens, file.clone()) {
        Ok(s) => s,
        Err(e) => return vec![error_to_diagnostic(source, &e)],
    };

    let mut checker = TypeChecker::new().with_source_file(file);
    checker.check(&statements);

    checker
        .errors
        .iter()
        .map(|e| error_to_diagnostic(source, e))
        .collect()
}
