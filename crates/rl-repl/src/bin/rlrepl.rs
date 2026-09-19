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
    name = "rlrepl",
    version,
    about = "Interactive REPL for the rl-lang programming language",
    styles = RL_STYLES
)]
struct Cli;

fn main() {
    let _cli = Cli::parse();
    rl_repl::start_vm_repl();
}
