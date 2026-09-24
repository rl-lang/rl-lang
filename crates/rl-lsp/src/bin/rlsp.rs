use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

const RL_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Yellow.on_default())
    .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::Green.on_default())
    .invalid(AnsiColor::Red.on_default());

#[derive(Parser)]
#[command(
    name = "rlsp",
    version,
    about = "Language Server Protocol server for rl-lang",
    styles = RL_STYLES,
    long_about = "Start the Language Server Protocol server, communicating \
                  over stdio. Intended to be launched by an editor, not run \
                  directly by hand."
)]
struct Cli;

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
    rl_lsp::run_lsp().await;
}
