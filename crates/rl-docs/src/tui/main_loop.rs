use crate::{
    entry::{ConceptEntry, StdEntry},
    tui::{
        formatting::markdown_to_lines,
        types::Focus,
        utils::{build_items, filter_items},
    },
};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

pub fn run(
    terminal: &mut DefaultTerminal,
    std_entries: &[&StdEntry],
    concept_entries: &[&ConceptEntry],
    tutorial_entries: &[&ConceptEntry],
    initial_query: Option<&str>,
) -> std::io::Result<()> {
    let items = build_items(std_entries, concept_entries, tutorial_entries);
    let mut query = initial_query.unwrap_or("").to_string();
    let mut focus = Focus::List;
    let mut selected: usize = 0;
    let mut content_scroll: u16 = 0;
    let mut list_state = ListState::default();

    loop {
        let filtered = filter_items(&items, &query);
        if filtered.is_empty() {
            selected = 0;
        } else if selected >= filtered.len() {
            selected = filtered.len() - 1;
        }
        list_state.select(if filtered.is_empty() {
            None
        } else {
            Some(selected)
        });

        terminal.draw(|frame| {
            let area = frame.area();
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(1)])
                .split(area);

            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(32), Constraint::Percentage(68)])
                .split(outer[0]);

            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(3)])
                .split(cols[0]);

            // search box
            let search_border = match focus {
                Focus::Search => Style::default().fg(Color::Cyan),
                Focus::List => Style::default().fg(Color::DarkGray),
            };
            let search_line = if query.is_empty() && matches!(focus, Focus::List) {
                Line::from(Span::styled(
                    "/ to search...",
                    Style::default().fg(Color::DarkGray),
                ))
            } else {
                let mut spans = vec![Span::styled(
                    query.clone(),
                    Style::default().fg(Color::White),
                )];
                if matches!(focus, Focus::Search) {
                    spans.push(Span::styled("│", Style::default().fg(Color::Cyan)));
                }
                Line::from(spans)
            };
            let search_widget = Paragraph::new(search_line).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(search_border)
                    .title(Span::styled(
                        " search ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )),
            );
            frame.render_widget(search_widget, left[0]);

            // sidebar list
            let list_items: Vec<ListItem> = filtered
                .iter()
                .map(|&idx| {
                    let item = &items[idx];
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("{:>8} ", item.tag),
                            Style::default().fg(item.tag_color),
                        ),
                        Span::styled(item.label.clone(), Style::default().fg(Color::White)),
                    ]))
                })
                .collect();

            let list_widget = List::new(list_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray))
                        .title(Span::styled(
                            format!(" entries ({}) ", filtered.len()),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_stateful_widget(list_widget, left[1], &mut list_state);

            // content pane
            let selected_item = filtered.get(selected).map(|&idx| &items[idx]);
            let content_lines = match selected_item {
                Some(item) => markdown_to_lines(&item.content),
                None => vec![Line::from(Span::styled(
                    "no matches",
                    Style::default().fg(Color::DarkGray),
                ))],
            };
            let content_title = selected_item
                .map(|item| format!(" {} ", item.label))
                .unwrap_or_else(|| " docs ".to_string());
            let content_widget = Paragraph::new(content_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray))
                        .title(Span::styled(
                            content_title,
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )),
                )
                .wrap(Wrap { trim: false });

            let visible = cols[1].height.saturating_sub(2);
            let total = content_widget.line_count(cols[1].width) as u16;
            let max_scroll = total.saturating_sub(visible);
            if content_scroll > max_scroll {
                content_scroll = max_scroll;
            }
            let content_widget = content_widget.scroll((content_scroll, 0));
            frame.render_widget(content_widget, cols[1]);

            // footer
            let footer_text = match focus {
                Focus::Search => "type to filter  •  Enter/Esc back to list  •  Ctrl+C quit",
                Focus::List => {
                    "/ search  •  ↑↓/jk select  •  PgUp/PgDn scroll  •  g/G first/last  •  q quit"
                }
            };
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    footer_text,
                    Style::default().fg(Color::DarkGray),
                ))),
                outer[1],
            );
        })?;

        if let Event::Key(key) = event::read()? {
            match focus {
                Focus::Search => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    (_, KeyCode::Enter) => {
                        focus = Focus::List;
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Esc) => {
                        query.clear();
                        focus = Focus::List;
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Backspace) => {
                        query.pop();
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Char(c)) => {
                        query.push(c);
                        selected = 0;
                        content_scroll = 0;
                    }
                    _ => {}
                },
                Focus::List => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    (_, KeyCode::Char('q')) => break,
                    (_, KeyCode::Char('/')) => focus = Focus::Search,
                    (_, KeyCode::Esc) if !query.is_empty() => {
                        query.clear();
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                        selected = selected.saturating_sub(1);
                        content_scroll = 0;
                    }
                    (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                        selected = selected.saturating_add(1);
                        content_scroll = 0;
                    }
                    (_, KeyCode::Char('g')) => {
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Char('G')) => {
                        selected = usize::MAX;
                        content_scroll = 0;
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('d')) | (_, KeyCode::PageDown) => {
                        content_scroll = content_scroll.saturating_add(10);
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('u')) | (_, KeyCode::PageUp) => {
                        content_scroll = content_scroll.saturating_sub(10);
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
