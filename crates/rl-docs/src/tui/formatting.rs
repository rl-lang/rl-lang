use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

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

/// Converts a single entry's rendered Markdown into styled ratatui lines:
/// headers get bold/cyan treatment, fenced code blocks are colored green
/// with the fence itself replaced by a divider, bullets get a dim marker,
/// and everything else runs through [`parse_inline`] for `**bold**` spans.
pub fn markdown_to_lines(content: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code = false;

    for raw in content.lines() {
        // code blocks
        if raw.starts_with("```") {
            in_code = !in_code;
            lines.push(Line::from(Span::styled(
                "─".repeat(40),
                Style::default().fg(Color::DarkGray),
            )));
            continue;
        }
        // code in code blocks
        if in_code {
            lines.push(Line::from(Span::styled(
                format!("   {raw}"),
                Style::default().fg(Color::Green),
            )));
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

    lines
}
