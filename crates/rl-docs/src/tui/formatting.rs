use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use rl_lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use rl_utils::source::SourceFile;

/// Splits `text` on `**bold**` delimiters into styled spans, alternating
/// `base` and bold-cyan on every `**` boundary.
pub fn parse_inline(text: &str, base: Color) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut bold = false;
    for part in text.split("**") {
        if !part.is_empty() {
            let style = if bold {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(base)
            };
            spans.push(Span::styled(part.to_string(), style));
        }
        bold = !bold;
    }
    if spans.is_empty() {
        spans.push(Span::raw(String::new()));
    }
    spans
}

/// Maps a lexer [`TokenType`] to the color it should render as in the TUI.
fn token_color(token: &TokenType) -> Color {
    use TokenType::*;
    match token {
        // keywords
        Null | Fn | In | For | While | Return | Break | Continue | Get | From | If | Else
        | Const | Dec | As | Ok | Err | Match | Record | Tag | Loop => Color::Magenta,
        // type keywords
        Int | Float | Bool | String | Byte | Char | Array | Error | Result | Map | Set => {
            Color::LightBlue
        }
        // literals
        StringLiteral(_) | CharacterLiteral(_) => Color::Green,
        NumberLiteral(_) | FloatLiteral(_) | ByteLiteral(_) => Color::Yellow,
        BoolLiteral(_) => Color::Magenta,
        // identifiers
        Identifier(_) => Color::White,
        // everything else: delimiters, punctuation, operators, newline/eof
        _ => Color::DarkGray,
    }
}

/// Tokenizes `code` with `rl-lexer` and renders it as syntax-highlighted
/// ratatui [`Line`]s.
fn highlight_rl_code(code: &str) -> Vec<Line<'static>> {
    let source_file = SourceFile::new("<docs>", code.to_string());
    let tokens = match Tokenizer::lex(source_file) {
        Ok(tokens) => tokens,
        Err(_) => {
            return code
                .lines()
                .map(|raw| {
                    Line::from(Span::styled(
                        format!("   {raw}"),
                        Style::default().fg(Color::Green),
                    ))
                })
                .collect();
        }
    };

    // Walk the token spans, filling any gaps (whitespace, comments - trivia
    // carries no span of its own) with the original source text in a dim
    // comment-ish color, so nothing from the snippet is silently dropped.
    let mut pieces: Vec<(String, Color)> = Vec::new();
    let mut cursor = 0usize;
    let bytes_len = code.len();

    for tok in &tokens {
        if matches!(tok.token, TokenType::Eof) {
            break;
        }
        let start = tok.span.start.min(bytes_len);
        let end = tok.span.end.min(bytes_len);
        if start > cursor {
            pieces.push((code[cursor..start].to_string(), Color::DarkGray));
        }
        if end > start {
            pieces.push((code[start..end].to_string(), token_color(&tok.token)));
        }
        cursor = end.max(cursor);
    }
    if cursor < bytes_len {
        pieces.push((code[cursor..].to_string(), Color::DarkGray));
    }

    // Split the colored pieces into lines at '\n', starting each rendered
    // line with the same 3-space indent as the old plain rendering.
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut current: Vec<Span<'static>> = vec![Span::raw("   ")];
    let mut current_has_content = false;

    for (text, color) in pieces {
        for (i, part) in text.split('\n').enumerate() {
            if i > 0 {
                lines.push(Line::from(std::mem::take(&mut current)));
                current.push(Span::raw("   "));
                current_has_content = false;
            }
            if !part.is_empty() {
                current.push(Span::styled(part.to_string(), Style::default().fg(color)));
                current_has_content = true;
            }
        }
    }
    if current_has_content || lines.is_empty() {
        lines.push(Line::from(current));
    }

    lines
}

/// Converts a single entry's rendered Markdown into styled ratatui lines:
/// headers get bold/cyan treatment, fenced code blocks are syntax
/// highlighted via `rl-lexer` with the fence itself replaced by a divider,
/// bullets get a dim marker, and everything else runs through
/// [`parse_inline`] for `**bold**` spans.
pub fn markdown_to_lines(content: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code = false;
    let mut code_buf: Vec<&str> = Vec::new();

    for raw in content.lines() {
        // code blocks
        if raw.starts_with("```") {
            if in_code {
                // closing fence: flush buffered code through the highlighter
                lines.extend(highlight_rl_code(&code_buf.join("\n")));
                code_buf.clear();
            }
            in_code = !in_code;
            lines.push(Line::from(Span::styled(
                "─".repeat(40),
                Style::default().fg(Color::DarkGray),
            )));
            continue;
        }
        // code in code blocks: buffer until the closing fence so the whole
        // block can be lexed together (tokens can span line boundaries).
        if in_code {
            code_buf.push(raw);
            continue;
        }
        // ---headings---
        // heading 4
        if let Some(rest) = raw.strip_prefix("#### ") {
            lines.push(Line::from(Span::styled(
                format!(" {rest}"),
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )));
        }
        // heading 3
        else if let Some(rest) = raw.strip_prefix("### ") {
            lines.push(Line::from(Span::styled(
                format!(" {rest}"),
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )));
        }
        // heading 2
        else if let Some(rest) = raw.strip_prefix("## ") {
            lines.push(Line::from(Span::styled(
                format!("- {rest}"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
        }
        // heading 1
        else if let Some(rest) = raw.strip_prefix("# ") {
            lines.push(Line::from(Span::styled(
                rest.to_string(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )));
        } else if let Some(rest) = raw.trim_start().strip_prefix("- ") {
            let indent = " ".repeat(raw.len() - raw.trim_start().len());
            let mut spans = vec![
                Span::raw(indent),
                Span::styled("  • ", Style::default().fg(Color::DarkGray)),
            ];
            spans.extend(parse_inline(rest, Color::White));
            lines.push(Line::from(spans));
        } else if raw.starts_with("output:") {
            lines.push(Line::from(Span::styled(
                raw.to_string(),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )));
        } else if raw.trim().is_empty() {
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(parse_inline(&format!(" {raw}"), Color::White)));
        }
    }

    // Unterminated code block (shouldn't normally happen): flush whatever
    // was buffered rather than silently dropping it.
    if in_code && !code_buf.is_empty() {
        lines.extend(highlight_rl_code(&code_buf.join("\n")));
    }

    lines
}
