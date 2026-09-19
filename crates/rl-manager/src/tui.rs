use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::error::Result;
use crate::install;
use crate::platform::{Arch, Platform};
use crate::variants::Variant;

struct App {
    variants: Vec<Variant>,
    state: ListState,
    version: String,
    install_dir: std::path::PathBuf,
    platform: Platform,
    arch: Arch,
    mode: AppMode,
    progress: Vec<String>,
}

enum AppMode {
    SelectVersion,
    SelectVariants,
    Installing,
    Done,
}

pub fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let platform = Platform::detect();
    let arch = Arch::detect();
    let install_dir = crate::platform::default_install_dir();

    let mut app = App {
        variants: crate::variants::all_variants(),
        state: ListState::default(),
        version: "latest".to_string(),
        install_dir,
        platform,
        arch,
        mode: AppMode::SelectVersion,
        progress: Vec::new(),
    };
    app.state.select(Some(0));

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.mode {
                AppMode::SelectVersion => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Up => {
                        let i = app.state.selected().map_or(0, |i| i.saturating_sub(1));
                        app.state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = app.state.selected().map_or(0, |i| i + 1);
                        app.state.select(Some(i.min(2)));
                    }
                    KeyCode::Enter => {
                        let selected = app.state.selected().unwrap_or(0);
                        app.version = match selected {
                            0 => "latest".to_string(),
                            1 => "nightly".to_string(),
                            2 => {
                                // For custom, fall back to "latest" for now
                                // A full TUI would have a text input here
                                "latest".to_string()
                            }
                            _ => "latest".to_string(),
                        };
                        app.mode = AppMode::SelectVariants;
                        app.state.select(Some(0));
                    }
                    _ => {}
                },
                AppMode::SelectVariants => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Up => {
                        let i = app.state.selected().map_or(0, |i| i.saturating_sub(1));
                        app.state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = app.state.selected().map_or(0, |i| i + 1);
                        app.state.select(Some(i.min(app.variants.len() - 1)));
                    }
                    KeyCode::Char('a') => {
                        // Select all
                        app.mode = AppMode::Installing;
                    }
                    KeyCode::Enter => {
                        app.mode = AppMode::Installing;
                    }
                    _ => {}
                },
                AppMode::Installing => {
                    // Run installation
                    install_all_tui(app)?;
                    app.mode = AppMode::Done;
                }
                AppMode::Done => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => return Ok(()),
                    _ => {}
                },
            }
        }
    }
}

fn install_all_tui(app: &mut App) -> Result<()> {
    let selected_idx = app.state.selected().unwrap_or(0);
    let variant = &app.variants[selected_idx];

    let mut progress = Vec::new();
    let mut progress_cb = |msg: &str| {
        progress.push(msg.to_string());
    };

    let _ = install::install_binary(
        variant,
        &app.version,
        app.platform,
        app.arch,
        &app.install_dir,
        false,
        Some(&mut progress_cb),
    );

    app.progress.append(&mut progress);
    app.progress.push("Done!".to_string());
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("  rlm", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" - rl-lang toolchain manager"),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, chunks[0]);

    // Main content
    match app.mode {
        AppMode::SelectVersion => render_version_picker(f, app, chunks[1]),
        AppMode::SelectVariants => render_variant_picker(f, app, chunks[1]),
        AppMode::Installing | AppMode::Done => render_progress(f, app, chunks[1]),
    }

    // Footer
    let footer_text = match app.mode {
        AppMode::SelectVersion => "↑↓ navigate  Enter select  q quit",
        AppMode::SelectVariants => "↑↓ navigate  Enter install  a install all  q quit",
        AppMode::Installing => "installing...",
        AppMode::Done => "press Enter or q to quit",
    };
    let footer = Paragraph::new(Line::from(Span::styled(
        format!("  {}", footer_text),
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(footer, chunks[2]);
}

fn render_version_picker(f: &mut Frame, app: &mut App, area: Rect) {
    let items = vec![
        ListItem::new(Line::from(vec![
            Span::styled("  latest", Style::default().fg(Color::Cyan)),
            Span::raw("  - newest stable release"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  nightly", Style::default().fg(Color::Cyan)),
            Span::raw(" - latest build from dev branch"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  custom", Style::default().fg(Color::Cyan)),
            Span::raw("  - pin a specific version"),
        ])),
    ];

    let list = List::new(items)
        .block(Block::default().title("  Select version").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    f.render_stateful_widget(list, area, &mut app.state);
}

fn render_variant_picker(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .variants
        .iter()
        .map(|v| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("  {:24}", v.name),
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(
                    format!("({})", v.actual),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(
                    "  Select variants to install (version: {})",
                    app.version
                ))
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    f.render_stateful_widget(list, area, &mut app.state);
}

fn render_progress(f: &mut Frame, app: &mut App, area: Rect) {
    let lines: Vec<Line> = app
        .progress
        .iter()
        .map(|msg| Line::from(Span::raw(format!("  {}", msg))))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("  Installation").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}
