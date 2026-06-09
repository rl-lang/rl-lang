use super::{
    command_handler::handle_command, depth_checker::is_complete, input_eval::eval_input,
    lines_types::OutputLine, output_render::render_output, syntax_highlighting::highlight,
    utils::char_to_byte,
};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::path::PathBuf;

use crate::interpreter::evaluator::Evaluator;

pub fn run_repl(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut evaluator = Evaluator::default().with_stdlib();
    let mut output: Vec<OutputLine> = vec![
        OutputLine::Info(format!(
            "rl-lang v{} — type :help for commands",
            env!("CARGO_PKG_VERSION")
        )),
        OutputLine::Separator,
    ];

    let mut input_buf = String::new(); // current line being typed
    let mut accumulated = String::new(); // multiline accumulator
    let mut cursor_pos: usize = 0; // cursor position in chars
    let mut history: Vec<String> = Vec::new();
    let mut history_idx: Option<usize> = None;
    let mut attached: Vec<PathBuf> = Vec::new();
    let mut scroll_offset: usize = 0;

    loop {
        // draw
        let is_continuation = !accumulated.is_empty();
        let prompt = if is_continuation { ".. " } else { ">> " };

        terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(area);

            // output area
            let out_lines = render_output(&output);
            let total = out_lines.len();
            let visible = chunks[0].height.saturating_sub(2) as usize;
            let max_scroll = total.saturating_sub(visible);
            // scroll_offset=0 means bottom and larger values scroll up
            let scroll = max_scroll.saturating_sub(scroll_offset) as u16;

            let output_widget = Paragraph::new(out_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray))
                        .title(Span::styled(
                            " rl ",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )),
                )
                .wrap(Wrap { trim: false })
                .scroll((scroll, 0));
            frame.render_widget(output_widget, chunks[0]);

            // input area
            let prompt_color = if is_continuation {
                Color::Yellow
            } else {
                Color::Cyan
            };
            let mut input_spans = vec![Span::styled(
                prompt,
                Style::default()
                    .fg(prompt_color)
                    .add_modifier(Modifier::BOLD),
            )];

            if input_buf.is_empty() {
                // blinking style cursor on empty input
                input_spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            } else {
                // split at char boundary safe for any unicode
                let before: String = input_buf.chars().take(cursor_pos).collect();
                let mut after_chars = input_buf.chars().skip(cursor_pos);

                let mut hl = highlight(&before);
                input_spans.append(&mut hl);

                match after_chars.next() {
                    None => {
                        // cursor past end of text
                        input_spans.push(Span::styled(
                            " ",
                            Style::default().bg(Color::Cyan).fg(Color::Black),
                        ));
                    }
                    Some(c) => {
                        let cursor_str = c.to_string();
                        let rest: String = after_chars.collect();
                        input_spans.push(Span::styled(
                            cursor_str,
                            Style::default().bg(Color::Cyan).fg(Color::Black),
                        ));
                        let mut hl2 = highlight(&rest);
                        input_spans.append(&mut hl2);
                    }
                }
            }

            let input_widget = Paragraph::new(Line::from(input_spans)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
            frame.render_widget(input_widget, chunks[1]);
        })?;

        // events
        if let Event::Key(key) = event::read()? {
            match (key.modifiers, key.code) {
                // exit
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,

                // submit
                (_, KeyCode::Enter) => {
                    let line = input_buf.clone();
                    input_buf.clear();
                    cursor_pos = 0;
                    history_idx = None;
                    scroll_offset = 0;

                    if line.trim().is_empty() && accumulated.is_empty() {
                        continue;
                    }

                    // commands only at top level (not inside a multiline block)
                    if accumulated.is_empty() && line.trim().starts_with(':') {
                        if line.trim() == ":exit" {
                            break;
                        }
                        output.push(OutputLine::Input(line.clone()));
                        handle_command(line.trim(), &mut output, &mut evaluator, &mut attached);
                        if !line.trim().is_empty() {
                            history.push(line);
                        }
                        continue;
                    }

                    // record which prompt was shown for this line before we push to accumulated
                    let is_first_line = accumulated.is_empty();

                    if !accumulated.is_empty() {
                        accumulated.push('\n');
                    }
                    accumulated.push_str(&line);

                    // first line: render_output adds ">> " prefix automatically via Input variant
                    // continuation lines: prepend ".. " inside the string so it shows correctly
                    if is_first_line {
                        output.push(OutputLine::Input(line.clone()));
                    } else {
                        output.push(OutputLine::Input(format!(".. {}", line)));
                    }

                    if is_complete(&accumulated)
                        && (accumulated.lines().count() > 1
                            || !accumulated.trim_end().ends_with('{'))
                    {
                        let full = accumulated.clone();
                        accumulated.clear();
                        if !full.trim().is_empty() {
                            history.push(full.trim().to_string());
                        }

                        let success = eval_input(full.trim(), &mut evaluator, &mut output);
                        if success {
                            output.push(OutputLine::ValidInput(full.trim().to_string()));
                        }
                    }
                }

                // backspace
                (_, KeyCode::Backspace) if cursor_pos > 0 => {
                    let byte_pos = char_to_byte(&input_buf, cursor_pos - 1);
                    input_buf.remove(byte_pos);
                    cursor_pos -= 1;
                }

                // delete
                (_, KeyCode::Delete) if cursor_pos < input_buf.chars().count() => {
                    let byte_pos = char_to_byte(&input_buf, cursor_pos);
                    input_buf.remove(byte_pos);
                }

                // word jump (ctrl+left / ctrl+right)
                (KeyModifiers::CONTROL, KeyCode::Left) => {
                    let chars: Vec<char> = input_buf.chars().collect();
                    let mut i = cursor_pos;
                    while i > 0 && chars[i - 1] == ' ' {
                        i -= 1;
                    }
                    while i > 0 && chars[i - 1] != ' ' {
                        i -= 1;
                    }
                    cursor_pos = i;
                }
                (KeyModifiers::CONTROL, KeyCode::Right) => {
                    let chars: Vec<char> = input_buf.chars().collect();
                    let len = chars.len();
                    let mut i = cursor_pos;
                    while i < len && chars[i] != ' ' {
                        i += 1;
                    }
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    cursor_pos = i;
                }

                // cursor movement
                (_, KeyCode::Left) if cursor_pos > 0 => {
                    cursor_pos = cursor_pos.saturating_sub(1);
                }

                (_, KeyCode::Right) if cursor_pos < input_buf.chars().count() => {
                    cursor_pos += 1;
                }

                (_, KeyCode::Home) => cursor_pos = 0,

                (_, KeyCode::End) => cursor_pos = input_buf.chars().count(),

                // scroll output (shift+up/down)
                (KeyModifiers::SHIFT, KeyCode::Up) => {
                    scroll_offset += 1;
                }

                (KeyModifiers::SHIFT, KeyCode::Down) => {
                    scroll_offset = scroll_offset.saturating_sub(1);
                }

                // history up
                (_, KeyCode::Up) => {
                    if history.is_empty() {
                        continue;
                    }
                    let new_idx = match history_idx {
                        None => history.len() - 1,
                        Some(0) => 0,
                        Some(i) => i - 1,
                    };
                    history_idx = Some(new_idx);
                    input_buf = history[new_idx].clone();
                    cursor_pos = input_buf.chars().count();
                }

                // history down
                (_, KeyCode::Down) => match history_idx {
                    None => {}
                    Some(i) if i + 1 >= history.len() => {
                        history_idx = None;
                        input_buf.clear();
                        cursor_pos = 0;
                    }
                    Some(i) => {
                        history_idx = Some(i + 1);
                        input_buf = history[i + 1].clone();
                        cursor_pos = input_buf.chars().count();
                    }
                },

                // escape — cancel multiline accumulation
                (_, KeyCode::Esc) => {
                    if !accumulated.is_empty() {
                        accumulated.clear();
                        output.push(OutputLine::Info("cancelled".into()));
                    }
                    input_buf.clear();
                    cursor_pos = 0;
                }

                // normal character input
                (_, KeyCode::Char(c)) => {
                    let byte_pos = char_to_byte(&input_buf, cursor_pos);
                    input_buf.insert(byte_pos, c);
                    cursor_pos += 1;
                }

                _ => {}
            }
        }
    }

    Ok(())
}
