//! Diagnostic pipeline: lex -> parse -> type-check.
//!
//! The evaluator is intentionally excluded. Running user code on every
//! keystroke caused the LSP to hang on infinite loops (e.g. `while true {}`).
//! The [`TypeChecker`] walks the AST without executing anything, making it
//! always safe to run on in-progress or non-terminating source.
use crate::to_diagnostic::error_to_diagnostic;
use rl_checker::TypeChecker;
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;
use tower_lsp::lsp_types::{Diagnostic, Url};

/// lex -> parse -> type-check the given source string and return LSP diagnostics.
///
/// the evaluator is no longer called here: running user code on every keystroke
/// caused the LSP to hang on infinite loops (e.g. `while true {}`). The
/// [`TypeChecker`] walks the same AST without executing anything, so it is
/// always safe to run on in-progress or even non-terminating source.
pub fn run_pipeline(source: &str, uri: &Url) -> Vec<Diagnostic> {
    let file_name = uri
        .to_file_path()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "buffer".to_string());
    let file = SourceFile::new(file_name, source.to_string());

    let tokens = match Tokenizer::lex(file.clone()) {
        Ok(t) => t,
        Err(e) => return vec![error_to_diagnostic(source, &e)],
    };

    let (ast, statements) = match Parser::parse(tokens, file.clone()) {
        Ok(s) => s,
        Err(e) => return vec![error_to_diagnostic(source, &e)],
    };

    let base_dir = uri
        .to_file_path()
        .ok()
        .and_then(|p| p.parent().map(std::path::Path::to_path_buf))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let mut checker = TypeChecker::new()
        .with_source_file(file)
        .with_ast_arena(ast)
        .with_base_dir(base_dir);
    if let Ok(doc_path) = uri.to_file_path()
        && let Some(doc_dir) = doc_path.parent()
    {
        checker = checker.with_base_dir(doc_dir.to_path_buf());
    }
    checker.check(&statements);

    checker
        .errors
        .iter()
        .map(|e| error_to_diagnostic(source, e))
        .collect()
}
