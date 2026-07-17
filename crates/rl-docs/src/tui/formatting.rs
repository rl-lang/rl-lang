use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

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

