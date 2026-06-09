use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::repl::{lines_types::OutputLine, syntax_highlighting::highlight};

pub fn render_output(output: &[OutputLine]) -> Vec<Line<'static>> {
    output
        .iter()
        .map(|line| match line {
            OutputLine::Input(s) => {
                // strips ".. "
                let (prefix, code) = if let Some(stripped) = s.strip_prefix(".. ") {
                    (".. ", stripped)
                } else {
                    (">> ", s.as_str())
                };
                let mut spans = vec![Span::styled(
                    prefix,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )];
                spans.extend(highlight(code));
                Line::from(spans)
            }
            OutputLine::ValidInput(s) => {
                let (prefix, code) = if let Some(stripped) = s.strip_prefix(".. ") {
                    (".. ", stripped)
                } else {
                    (">> ", s.as_str())
                };
                let mut spans = vec![Span::styled(
                    prefix,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )];
                spans.extend(highlight(code));
                Line::from(spans)
            }
            OutputLine::Result(s) => {
                // try to highlight if it is correct
                let spans = highlight(s);
                // highlight returns red on lex failure
                let is_code = spans.iter().any(|sp| {
                    sp.style
                        .fg
                        .map(|c| c != Color::Red && c != Color::White)
                        .unwrap_or(false)
                });
                if is_code {
                    Line::from(spans)
                } else {
                    Line::from(Span::styled(s.clone(), Style::default().fg(Color::Green)))
                }
            }
            OutputLine::Error(s) => Line::from(vec![
                Span::styled(
                    "✗ ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(s.clone(), Style::default().fg(Color::Red)),
            ]),
            OutputLine::Info(s) => Line::from(Span::styled(
                s.clone(),
                Style::default().fg(Color::DarkGray),
            )),
            OutputLine::Separator => Line::from(Span::styled(
                "─".repeat(40),
                Style::default().fg(Color::DarkGray),
            )),
            OutputLine::Styled(parts) => Line::from(
                parts
                    .iter()
                    .map(|(text, style)| Span::styled(text.clone(), *style))
                    .collect::<Vec<_>>(),
            ),
        })
        .collect()
}
