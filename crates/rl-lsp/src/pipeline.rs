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

/// lex -> parse -> type-check the given source string, returning
/// diagnostics grouped by owning file URI.
///
/// Errors raised while checking `get ... from <file>` imports carry the
/// imported file's name and text (see `TypeChecker::import_module`), so
/// their offsets must be mapped with their own source - never the open
/// document's. The first group is always the document itself (possibly
/// empty); the rest are imported files, also possibly empty.
pub fn run_pipeline(source: &str, uri: &Url) -> Vec<(Url, Vec<Diagnostic>)> {
    let file_name = uri
        .to_file_path()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "buffer".to_string());
    let file = SourceFile::new(file_name.clone(), source.to_string());

    let tokens = match Tokenizer::lex(file.clone()) {
        Ok(t) => t,
        Err(e) => return vec![(uri.clone(), vec![error_to_diagnostic(source, &e)])],
    };

    let (ast, statements) = match Parser::parse(tokens, file.clone()) {
        Ok(s) => s,
        Err(e) => return vec![(uri.clone(), vec![error_to_diagnostic(source, &e)])],
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

    // group by owning file; each error maps offsets with its own text
    let mut groups: Vec<(Url, Vec<Diagnostic>)> = vec![(uri.clone(), Vec::new())];
    for e in checker.warnings.iter().chain(checker.errors.iter()) {
        let text = e.source_text().map(|s| s.as_str()).unwrap_or(source);
        let diag = error_to_diagnostic(text, e);
        let idx = group_index(&mut groups, uri, &file_name, e.source_name());
        groups[idx].1.push(diag);
    }
    groups
}

/// Index of the diagnostics group for `name` (`None` or the document's
/// own name go to group 0), creating it when missing. A path that is
/// not a URI falls back to the document (legacy behavior); offsets may
/// be off, but nothing is silently dropped.
fn group_index(
    groups: &mut Vec<(Url, Vec<Diagnostic>)>,
    uri: &Url,
    file_name: &str,
    name: Option<&str>,
) -> usize {
    match name {
        None => 0,
        Some(n) if n == file_name => 0,
        Some(n) => {
            if let Some(pos) = groups.iter().position(|(u, _)| {
                u.to_file_path()
                    .ok()
                    .map(|p| p.to_string_lossy().into_owned())
                    .as_deref()
                    == Some(n)
            }) {
                pos
            } else {
                let file_uri = Url::from_file_path(n).unwrap_or_else(|_| uri.clone());
                if &file_uri == uri {
                    0
                } else {
                    groups.push((file_uri, Vec::new()));
                    groups.len() - 1
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run_pipeline;
    use tower_lsp::lsp_types::Url;

    /// Errors from `get ... from <file>` imports must publish under the
    /// imported file's URI with ranges mapped against its text - never
    /// as misplaced squiggles in the importing document.
    #[test]
    fn imported_errors_route_to_imported_uri() {
        let dir = tempfile::tempdir().unwrap();
        // line 1 is short on purpose: an offset mapped against the
        // wrong text would land on a different line here
        std::fs::write(
            dir.path().join("lib.rl"),
            "dec broken_thing = undefined_function_xyz()\ndec int ok_one = 1\ndec int ok_two = 2\ndec int ok_three = 3\n",
        )
        .unwrap();
        let main_src = "get broken_thing from lib\nget println from std::io\nprintln(\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\")\n";
        let main_path = dir.path().join("main.rl");
        std::fs::write(&main_path, main_src).unwrap();
        let main_uri = Url::from_file_path(&main_path).unwrap();

        let groups = run_pipeline(main_src, &main_uri);
        assert_eq!(groups.len(), 2, "expected main + lib groups: {groups:?}");
        assert_eq!(groups[0].0, main_uri);
        assert!(
            groups[0].1.is_empty(),
            "main.rl must not inherit lib.rl errors: {:?}",
            groups[0].1
        );

        let lib_uri = Url::from_file_path(dir.path().join("lib.rl")).unwrap();
        assert_eq!(groups[1].0, lib_uri);
        // the undefined-name error plus the unused-binding warning,
        // both attributed to lib.rl with lib.rl-relative ranges
        assert_eq!(groups[1].1.len(), 2);
        let diag = groups[1]
            .1
            .iter()
            .find(|d| d.message.contains("undefined_function_xyz"))
            .expect("expected the undefined-name error in lib.rl group");
        // undefined_function_xyz starts line 1 of lib.rl (0-based line 0);
        // mapped against main.rl's text it would land on line 2+
        assert_eq!(diag.range.start.line, 0, "range: {:?}", diag.range);
    }

    /// A clean document yields one empty group (clears stale squiggles).
    #[test]
    fn clean_document_yields_empty_group() {
        let dir = tempfile::tempdir().unwrap();
        let main_path = dir.path().join("main.rl");
        let src = "get println from std::io\nprintln(\"hi\")\n";
        std::fs::write(&main_path, src).unwrap();
        let uri = Url::from_file_path(&main_path).unwrap();

        let groups = run_pipeline(src, &uri);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].0, uri);
        assert!(groups[0].1.is_empty(), "{:?}", groups[0].1);
    }
}
