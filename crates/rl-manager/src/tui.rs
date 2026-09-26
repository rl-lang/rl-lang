use std::io;
use std::path::PathBuf;
use std::time::Duration;

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
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::error::Result;
use crate::install::{self, InstallEvent, PHASE_CHECKSUM, PHASE_DOWNLOAD, PHASE_EXTRACT};
use crate::platform::{Arch, Platform};
use crate::variants::Variant;

struct App {
    variants: Vec<Variant>,
    state: ListState,
    version: String,
    install_dir: std::path::PathBuf,
    force: bool,
    install_all: bool,
    platform: Platform,
    arch: Arch,
    mode: AppMode,
    /// Log lines shown on the Done screen.
    progress: Vec<String>,
    /// Final bar ratios shown on the Done screen: [download, sha256, extract].
    bars: [f64; 3],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppMode {
    SelectVersion,
    SelectVariants,
    Installing,
    Done,
}

/// Live install state, kept outside [`App`] while installing so the
/// progress callback can update it and redraw on every event.
#[derive(Default)]
struct LiveState {
    title: String,
    download: f64,
    checksum: f64,
    extract: f64,
    download_label: String,
    checksum_label: String,
    extract_label: String,
    log: Vec<String>,
}

pub fn run_tui(install_dir: PathBuf, force: bool) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let platform = Platform::detect();
    let arch = Arch::detect();

    let mut app = App {
        variants: crate::variants::all_variants(),
        state: ListState::default(),
        version: "latest".to_string(),
        install_dir,
        force,
        install_all: false,
        platform,
        arch,
        mode: AppMode::SelectVersion,
        progress: Vec::new(),
        bars: [0.0; 3],
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

        // Run the install immediately on entering the Installing state so
        // the user never sits on an empty panel waiting for another key.
        if app.mode == AppMode::Installing {
            install_all_tui(terminal, app)?;
            app.mode = AppMode::Done;
            continue;
        }

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
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
                        app.state.select(Some(i.min(app.variants.len().saturating_sub(1))));
                    }
                    KeyCode::Char('a') => {
                        // Install everything
                        app.install_all = true;
                        app.mode = AppMode::Installing;
                    }
                    KeyCode::Enter => {
                        app.install_all = false;
                        app.mode = AppMode::Installing;
                    }
                    _ => {}
                },
                AppMode::Installing => {}
                AppMode::Done => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => return Ok(()),
                    _ => {}
                },
            }
        }
    }
}

fn fmt_bytes(n: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    let f = n as f64;
    if f >= MB {
        format!("{:.1} MB", f / MB)
    } else if f >= KB {
        format!("{:.1} KB", f / KB)
    } else {
        format!("{} B", n)
    }
}

fn apply_event(live: &mut LiveState, ev: InstallEvent) {
    match ev {
        InstallEvent::Message(m) => live.log.push(m),
        InstallEvent::PhaseStart { phase } => {
            if phase == PHASE_EXTRACT {
                live.extract = 0.0;
                live.extract_label = "working...".to_string();
            }
        }
        InstallEvent::PhaseProgress { phase, done, total } => {
            let ratio = total
                .filter(|t| *t > 0)
                .map(|t| (done as f64 / t as f64).clamp(0.0, 1.0));
            if phase == PHASE_DOWNLOAD {
                if let Some(r) = ratio {
                    live.download = r;
                    live.download_label = format!(
                        "{} / {} ({}%)",
                        fmt_bytes(done),
                        fmt_bytes(total.unwrap_or(done)),
                        (r * 100.0) as u64
                    );
                } else {
                    live.download_label = fmt_bytes(done);
                }
            } else if phase == PHASE_CHECKSUM {
                if let Some(r) = ratio {
                    live.checksum = r;
                    live.checksum_label = format!("{}%", (r * 100.0) as u64);
                } else {
                    live.checksum_label = fmt_bytes(done);
                }
            }
        }
        InstallEvent::PhaseDone { phase } => {
            if phase == PHASE_DOWNLOAD {
                live.download = 1.0;
            } else if phase == PHASE_CHECKSUM {
                live.checksum = 1.0;
                live.checksum_label = "ok".to_string();
            } else if phase == PHASE_EXTRACT {
                live.extract = 1.0;
                live.extract_label = "done".to_string();
            }
        }
    }
    if live.log.len() > 500 {
        let overflow = live.log.len() - 500;
        live.log.drain(..overflow);
    }
}

fn install_all_tui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    // Copy everything the progress closure needs out of `app` so the
    // closure can redraw (which borrows `app` immutably) while it owns
    // no mutable borrow of `app`.
    let version_in = app.version.clone();
    let variants: Vec<Variant> = if app.install_all {
        app.variants.clone()
    } else {
        app.state
            .selected()
            .and_then(|i| app.variants.get(i).cloned())
            .into_iter()
            .collect()
    };
    let platform = app.platform;
    let arch = app.arch;
    let install_dir = app.install_dir.clone();
    let force = app.force;

    let mut live = LiveState::default();

    let resolved = match crate::version::resolve_version(&version_in) {
        Ok(v) => v,
        Err(e) => {
            app.progress = vec![format!("error: {}", e), "Done!".to_string()];
            app.bars = [0.0; 3];
            return Ok(());
        }
    };
    live.log.push(format!("version: {}", resolved));

    if variants.is_empty() {
        live.log.push("nothing selected".to_string());
    }

    for variant in &variants {
        live.title = format!("{}  {}", variant.name, resolved);
        live.download = 0.0;
        live.checksum = 0.0;
        live.extract = 0.0;
        live.download_label.clear();
        live.checksum_label.clear();
        live.extract_label.clear();
        live.log.push(format!("-- {} --", variant.name));
        draw_live(terminal, &live)?;

        let mut cb = |ev: InstallEvent| {
            apply_event(&mut live, ev);
            let _ = draw_live(terminal, &live);
        };

        match install::install_binary(
            variant,
            &resolved,
            platform,
            arch,
            &install_dir,
            force,
            Some(&mut cb),
        ) {
            Ok(_) => {}
            Err(e) => live.log.push(format!("[FAIL] {}: {}", variant.name, e)),
        }
    }

    live.log.push("Done!".to_string());
    app.bars = [live.download, live.checksum, live.extract];
    app.progress = live.log;
    Ok(())
}

/// Draw the live installing screen outside the normal `ui` flow so the
/// progress callback can refresh on every download/checksum event.
fn draw_live(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    live: &LiveState,
) -> Result<()> {
    terminal.draw(|f| {
        let area = f.area();
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);
        f.render_widget(header_widget(), chunks[0]);
        draw_installing(f, chunks[1], live);
        f.render_widget(footer_widget("installing..."), chunks[2]);
    })?;
    Ok(())
}

fn header_widget() -> Paragraph<'static> {
    Paragraph::new(Line::from(vec![
        Span::styled("  rlm", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" - rl-lang toolchain manager"),
    ]))
    .block(Block::default().borders(Borders::BOTTOM))
}

fn footer_widget(text: &str) -> Paragraph<'_> {
    Paragraph::new(Line::from(Span::styled(
        format!("  {}", text),
        Style::default().fg(Color::DarkGray),
    )))
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

    f.render_widget(header_widget(), chunks[0]);

    // Main content
    match app.mode {
        AppMode::SelectVersion => render_version_picker(f, app, chunks[1]),
        AppMode::SelectVariants => render_variant_picker(f, app, chunks[1]),
        AppMode::Installing => {
            // Transient: the install runs immediately, so this is only
            // visible for one frame before live draws take over.
            let live = LiveState::default();
            draw_installing(f, chunks[1], &live);
        }
        AppMode::Done => {
            let mut live = LiveState {
                download: app.bars[0],
                checksum: app.bars[1],
                extract: app.bars[2],
                ..Default::default()
            };
            live.log.clone_from(&app.progress);
            if live.download >= 1.0 && live.download_label.is_empty() {
                live.download_label = "done".to_string();
            }
            if live.checksum >= 1.0 && live.checksum_label.is_empty() {
                live.checksum_label = "ok".to_string();
            }
            if live.extract >= 1.0 && live.extract_label.is_empty() {
                live.extract_label = "done".to_string();
            }
            draw_installing(f, chunks[1], &live);
        }
    }

    // Footer
    let footer_text = match app.mode {
        AppMode::SelectVersion => "↑↓ navigate  Enter select  q quit",
        AppMode::SelectVariants => "↑↓ navigate  Enter install  a install all  q quit",
        AppMode::Installing => "installing...",
        AppMode::Done => "press Enter or q to quit",
    };
    f.render_widget(footer_widget(footer_text), chunks[2]);
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
        .block(Block::default().title(" Select version").borders(Borders::ALL))
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
                    " Select variants to install (version: {})",
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

fn gauge(title: &'static str, ratio: f64, label: &str) -> Gauge<'static> {
    Gauge::default()
        .block(Block::default().title(title).borders(Borders::ALL))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(ratio.clamp(0.0, 1.0))
        .label(Span::raw(label.to_string()))
}

fn draw_installing(f: &mut Frame, area: Rect, live: &LiveState) {
    let rows = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let title = if live.title.is_empty() {
        Line::from(Span::styled(
            " Installing...",
            Style::default().add_modifier(Modifier::BOLD),
        ))
    } else {
        // No quit hint here: keypresses aren't read while the install runs.
        Line::from(Span::styled(
            format!(" Installing {}", live.title),
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };
    f.render_widget(Paragraph::new(title), rows[0]);

    f.render_widget(
        gauge(" Download ", live.download, &live.download_label),
        rows[1],
    );
    f.render_widget(
        gauge(" SHA-256 ", live.checksum, &live.checksum_label),
        rows[2],
    );
    f.render_widget(
        gauge(" Extract ", live.extract, &live.extract_label),
        rows[3],
    );

    // Auto-scroll the log to the last visible lines.
    let visible = rows[4].height.saturating_sub(2).max(1) as usize;
    let start = live.log.len().saturating_sub(visible);
    let lines: Vec<Line> = live.log[start..]
        .iter()
        .map(|msg| Line::from(Span::raw(format!(" {}", msg))))
        .collect();

    let log = Paragraph::new(lines).block(Block::default().title(" Log ").borders(Borders::ALL));
    f.render_widget(log, rows[4]);
}
