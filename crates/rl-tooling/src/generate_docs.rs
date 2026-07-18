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
const STYLE_CSS: &str = include_str!("../assets/style.css");

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

/// Groups items by their source file, sorted by file name, ready to be
/// rendered into per-file pages by any output-format writer.
fn group_by_file(items: &[DocItem]) -> Vec<(&str, Vec<&DocItem>)> {
    let mut by_file: Vec<(&str, Vec<&DocItem>)> = Vec::new();
    for item in items {
        match by_file.iter_mut().find(|(f, _)| *f == item.file) {
            Some((_, v)) => v.push(item),
            None => by_file.push((&item.file, vec![item])),
        }
    }
    by_file.sort_by(|a, b| a.0.cmp(b.0));
    by_file
}

/// Writes an `index.md` plus one markdown page per source file into
/// `out_dir`, creating it if needed.
pub fn write_doc_site(items: &[DocItem], out_dir: &Path, project_name: &str) -> io::Result<()> {
    fs::create_dir_all(out_dir)?;

    let by_file = group_by_file(items);

    let mut index = format!("# {} docs\n\n", project_name);
    for (file, file_items) in &by_file {
        let page = page_name(file, "md");
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

/// Writes an `index.html` plus one HTML page per source file into
/// `out_dir`, creating it if needed. Shares a single `style.css` across
/// all pages. Mirrors [`write_doc_site`]'s layout so the two output
/// formats stay in sync.
///
/// `highlight_js`, if given, overrides the built-in `.rl` syntax
/// highlighter with a different script.
/// Pass `no_highlight: true` to omit highlighting entirely - otherwise
/// the built-in [`DEFAULT_HIGHLIGHT_JS`] is inlined automatically.
pub fn write_doc_site_html(
    items: &[DocItem],
    out_dir: &Path,
    project_name: &str,
    highlight_js: Option<&Path>,
    no_highlight: bool,
) -> io::Result<()> {
    fs::create_dir_all(out_dir)?;
    fs::write(out_dir.join("style.css"), STYLE_CSS)?;

    let script_content: Option<String> = if no_highlight {
        None
    } else if let Some(src_path) = highlight_js {
        Some(fs::read_to_string(src_path)?)
    } else {
        Some(DEFAULT_HIGHLIGHT_JS.to_string())
    };
    let script_content = script_content.as_deref();
    if let Some(js) = script_content { fs::write(out_dir.join("rl-highlight.js"), js)? }

    let by_file = group_by_file(items);

    let mut index_links = String::new();
    for (file, file_items) in &by_file {
        let page = page_name(file, "html");
        index_links.push_str(&format!(
            "<li><a href=\"{page}\">{file}</a> <span class=\"count\">({count} item{s})</span></li>\n",
            page = page,
            file = html_escape(file),
            count = file_items.len(),
            s = if file_items.len() == 1 { "" } else { "s" }
        ));

        let mut sections = String::new();
        for item in file_items {
            sections.push_str(&format!(
                "<section class=\"item\">\n\
                 <h2><code class=\"kind\">{kind}</code> {name} <span class=\"line\">line {line}</span></h2>\n\
                 <pre class=\"sig rl-code\">{sig}</pre>\n\
                 <p class=\"doc\">{doc}</p>\n\
                 </section>\n",
                kind = html_escape(item.kind),
                name = html_escape(&item.name),
                line = item.line,
                sig = html_escape(&item.signature),
                doc = html_escape(&item.doc).replace('\n', "<br>\n"),
            ));
        }

        let page_html = html_page(
            &format!("{} - {}", file, project_name),
            &format!("<h1>{}</h1>\n{}", html_escape(file), sections),
            script_content,
        );
        fs::write(out_dir.join(page_name(file, "html")), page_html)?;
    }

    let index_html = html_page(
        &format!("{} docs", project_name),
        &format!(
            "<h1>{} docs</h1>\n<ul class=\"file-list\">\n{}</ul>\n",
            html_escape(project_name),
            index_links
        ),
        script_content,
    );
    fs::write(out_dir.join("index.html"), index_html)?;
    Ok(())
}

/// Wraps a `<body>` fragment in a minimal HTML5 document that links the
/// shared `style.css` and, if given, inlines a syntax-highlighter script.
fn html_page(title: &str, body: &str, script_content: Option<&str>) -> String {
    let script_tag = match script_content {
        Some(_) => "<script src=\"rl-highlight.js\"></script>\n".to_string(),
        None => String::new(),
    };
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{title}</title>\n\
         <link rel=\"stylesheet\" href=\"style.css\">\n\
         </head>\n<body>\n{body}{script_tag}</body>\n</html>\n",
        title = html_escape(title),
        body = body,
        script_tag = script_tag,
    )
}

/// Escapes the five characters that are meaningful in HTML text content.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

fn page_name(file: &str, ext: &str) -> String {
    let stem = Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("page");
    format!("{}.{}", stem, ext)
}
