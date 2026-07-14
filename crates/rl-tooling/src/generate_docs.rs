//! Generates a project documentation site from `///` doc comments in `.rl`
//! source, similar in spirit to rustdoc.
//!
//! A `///` block directly above a `fn`, `record`, `tag`, `const`, or `dec`
//! declaration is picked up as that item's documentation. Plain `//`
//! comments are ignored.

use std::fs;
use std::io;
use std::path::Path;

use rl_lexer::tokentypes::{Token, TokenType, Trivia};

use crate::format::needs_space;

/// The default `.rl` syntax highlighter, compiled into the binary and
/// inlined into generated pages unless `--no-highlight` is passed or a
/// different script is supplied via `--highlight-js`.
const DEFAULT_HIGHLIGHT_JS: &str = include_str!("../assets/rl-highlight.js");

/// One documented item extracted from a source file.
pub struct DocItem {
    pub kind: &'static str,
    pub name: String,
    pub signature: String,
    pub doc: String,
    pub file: String,
    pub line: usize,
}

/// Scans `tokens` (from one source file) for declarations immediately
/// preceded by `///` doc comments and returns the extracted items.
pub fn extract_doc_items(tokens: &[Token], file_name: &str) -> Vec<DocItem> {
    let mut items = Vec::new();

    for (i, tok) in tokens.iter().enumerate() {
        let kind = match &tok.token {
            TokenType::Fn => "fn",
            TokenType::Record => "record",
            TokenType::Tag => "tag",
            TokenType::Const => "const",
            TokenType::Dec => "dec",
            _ => continue,
        };

        let doc_lines: Vec<&str> = tok
            .leading_trivia
            .iter()
            .filter_map(|t| match t {
                Trivia::DocComment(c) => Some(c.as_str()),
                _ => None,
            })
            .collect();

        if doc_lines.is_empty() {
            continue;
        }

        let name = match tokens.get(i + 1).map(|t| &t.token) {
            Some(TokenType::Identifier(n)) => n.clone(),
            _ => continue,
        };

        let signature = render_signature(tokens, i);

        items.push(DocItem {
            kind,
            name,
            signature,
            doc: doc_lines.join("\n"),
            file: file_name.to_string(),
            line: tok.line,
        });
    }

    items
}

/// Reconstructs a readable one-line-ish signature starting at the
/// declaration keyword, stopping at the opening `{` (fn/record/tag body) or
/// at `;`/newline (const/dec), whichever comes first.
fn render_signature(tokens: &[Token], start: usize) -> String {
    let mut out = String::new();
    let mut prev: Option<&TokenType> = None;

    for tok in &tokens[start..] {
        match &tok.token {
            TokenType::LeftBrace | TokenType::Semicolon | TokenType::Newline | TokenType::Eof => {
                break;
            }
            other => {
                if let Some(p) = prev
                    && needs_space(p, other)
                {
                    out.push(' ');
                }
                out.push_str(&tok.lexeme);
                prev = Some(other);
            }
        }
    }

    out
}

/// Writes an `index.md` plus one markdown page per source file into
/// `out_dir`, creating it if needed.
pub fn write_doc_site(items: &[DocItem], out_dir: &Path, project_name: &str) -> io::Result<()> {
    fs::create_dir_all(out_dir)?;

    let mut by_file: Vec<(&str, Vec<&DocItem>)> = Vec::new();
    for item in items {
        match by_file.iter_mut().find(|(f, _)| *f == item.file) {
            Some((_, v)) => v.push(item),
            None => by_file.push((&item.file, vec![item])),
        }
    }
    by_file.sort_by(|a, b| a.0.cmp(b.0));

    let mut index = format!("# {} docs\n\n", project_name);
    for (file, file_items) in &by_file {
        let page = page_name(file);
        index.push_str(&format!("- [{}]({})\n", file, page));

        let mut page_out = format!("# {}\n\n", file);
        for item in file_items {
            page_out.push_str(&format!(
                "## `{}` {} (line {})\n\n```rl\n{}\n```\n\n{}\n\n",
                item.kind, item.name, item.line, item.signature, item.doc
            ));
        }
        fs::write(out_dir.join(&page), page_out)?;
    }

    fs::write(out_dir.join("index.md"), index)?;
    Ok(())
}

fn page_name(file: &str) -> String {
    let stem = Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("page");
    format!("{}.md", stem)
}
