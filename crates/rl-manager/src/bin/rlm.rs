use std::path::PathBuf;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};

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
    name = "rlm",
    version,
    about = "rl-lang toolchain manager",
    styles = RL_STYLES,
    after_help = "EXAMPLES:\n    \
                   rlm install                    # interactive install\n    \
                   rlm install latest              # install latest stable\n    \
                   rlm install nightly             # install nightly\n    \
                   rlm install v2.2.0              # install specific version\n    \
                   rlm install --variant rl,rl_vm  # install specific variants\n    \
                   rlm install --no-tui            # CLI-only mode\n    \
                   rlm update                      # update rlm itself\n    \
                   rlm uninstall                   # remove installed binaries\n    \
                   rlm list                        # list installed binaries"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install rl-lang binaries from GitHub Releases
    Install {
        /// Version to install (latest, nightly, or v2.2.0)
        version: Option<String>,

        /// Comma-separated list of variants to install (or "all")
        #[arg(short, long)]
        variant: Option<String>,

        /// Install directory (default: ~/.local/bin)
        #[arg(short, long)]
        prefix: Option<PathBuf>,

        /// Overwrite existing binaries without prompting
        #[arg(short, long)]
        force: bool,

        /// Use CLI mode instead of TUI
        #[arg(long)]
        no_tui: bool,
    },

    /// Update rlm itself to the latest version
    Update,

    /// Remove installed rl-lang binaries
    Uninstall,

    /// List installed rl-lang binaries
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { version, variant, prefix, force, no_tui } => {
            cmd_install(version, variant, prefix, force, no_tui);
        }
        Commands::Update => {
            cmd_update();
        }
        Commands::Uninstall => {
            cmd_uninstall();
        }
        Commands::List => {
            cmd_list();
        }
    }
}

fn cmd_install(
    version: Option<String>,
    variant: Option<String>,
    prefix: Option<PathBuf>,
    force: bool,
    no_tui: bool,
) {
    let platform = rl_manager::Platform::detect();
    let arch = rl_manager::Arch::detect();
    let install_dir = prefix.unwrap_or_else(rl_manager::platform::default_install_dir);

    // Try TUI if available and not disabled
    #[cfg(feature = "tui")]
    {
        if !no_tui && std::io::stdin().is_terminal() {
            if let Err(e) = rl_manager::tui::run_tui() {
                eprintln!("TUI error: {}", e);
                std::process::exit(1);
            }
            return;
        }
    }

    // CLI fallback
    let version_input = version
        .or_else(|| std::env::var("RL_VERSION").ok())
        .unwrap_or_else(|| {
            if no_tui || !std::io::stdin().is_terminal() {
                "latest".to_string()
            } else {
                pick_version_interactive()
            }
        });

    let resolved = match rl_manager::version::resolve_version(&version_input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    // Select variants
    let selected = select_variants(variant, no_tui);
    if selected.is_empty() {
        eprintln!("error: no variants selected");
        std::process::exit(1);
    }

    println!();
    println!("  rl-lang installer");
    println!("  repo:    {}", rl_manager::variants::REPO);
    println!("  arch:    {}", arch.as_str());
    println!("  platform:{}", platform.as_str());
    println!("  version: {}", resolved);
    println!("  install: {}", install_dir.display());
    println!("  ----------------------------------------");
    println!();

    let mut installed = 0;
    let total = selected.len();

    for variant in &selected {
        let mut progress_cb = |msg: &str| {
            println!("  {}", msg);
        };

        match rl_manager::install::install_binary(
            variant,
            &resolved,
            platform,
            arch,
            &install_dir,
            force,
            Some(&mut progress_cb),
        ) {
            Ok(true) => installed += 1,
            Ok(false) => {}
            Err(e) => {
                eprintln!("  [FAIL] {}: {}", variant.name, e);
            }
        }
    }

    println!();
    if installed == total {
        println!("  Summary: {}/{} installed.", installed, total);
    } else {
        println!("  Summary: {}/{} installed, some failed.", installed, total);
    }

    // Check PATH
    let path_str = std::env::var("PATH").unwrap_or_default();
    if !path_str.split(':').any(|p| p == install_dir.to_string_lossy().as_ref()) {
        println!();
        println!("  Add this to your shell profile:");
        println!("    export PATH=\"{}:$PATH\"", install_dir.display());
    }
}

fn cmd_update() {
    let platform = rl_manager::Platform::detect();
    let arch = rl_manager::Arch::detect();
    let install_dir = rl_manager::platform::default_install_dir();

    // Find the rlm binary
    let rlm_name = if platform == rl_manager::Platform::Windows {
        "rlm.exe"
    } else {
        "rlm"
    };

    let rlm_path = install_dir.join(rlm_name);
    if !rlm_path.exists() {
        eprintln!("error: rlm not found at {}", rlm_path.display());
        eprintln!("       cannot self-update without knowing install location");
        std::process::exit(1);
    }

    println!("  Updating rlm...");

    // Use the rlm binary to install the latest rlm variant
    // For now, download the latest release and replace
    let variant = rl_manager::variants::Variant {
        name: "rlm",
        actual: "rlm",
        group: "Manager",
    };

    let mut progress_cb = |msg: &str| {
        println!("  {}", msg);
    };

    match rl_manager::install::install_binary(
        &variant,
        "latest",
        platform,
        arch,
        &install_dir,
        true,
        Some(&mut progress_cb),
    ) {
        Ok(_) => println!("  rlm updated successfully."),
        Err(e) => {
            eprintln!("  failed to update rlm: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_uninstall() {
    let install_dir = rl_manager::platform::default_install_dir();
    let platform = rl_manager::Platform::detect();
    let mut removed = 0;

    println!();
    println!("  Uninstalling rl-lang binaries from {}...", install_dir.display());
    println!();

    for variant in rl_manager::variants::all_variants() {
        let names = if platform == rl_manager::Platform::Windows {
            vec![format!("{}.exe", variant.actual)]
        } else {
            vec![variant.actual.to_string()]
        };

        for name in &names {
            let path = install_dir.join(name);
            if path.exists() {
                match std::fs::remove_file(&path) {
                    Ok(_) => {
                        println!("  [ OK ] Removed: {}", path.display());
                        removed += 1;
                    }
                    Err(e) => {
                        eprintln!("  [FAIL] Could not remove {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    println!();
    if removed > 0 {
        println!("  Removed {} binary(ies).", removed);
    } else {
        println!("  No rl-lang binaries found in {}.", install_dir.display());
    }
}

fn cmd_list() {
    let install_dir = rl_manager::platform::default_install_dir();
    let platform = rl_manager::Platform::detect();

    println!();
    println!("  Installed rl-lang binaries in {}:", install_dir.display());
    println!();

    let mut found = false;
    for variant in rl_manager::variants::all_variants() {
        let names = if platform == rl_manager::Platform::Windows {
            vec![format!("{}.exe", variant.actual)]
        } else {
            vec![variant.actual.to_string()]
        };

        for name in &names {
            let path = install_dir.join(name);
            if path.exists() {
                println!("  {} ({})", variant.name, name);
                found = true;
            }
        }
    }

    if !found {
        println!("  No rl-lang binaries found.");
    }
    println!();
}

fn pick_version_interactive() -> String {
    use std::io::{self, Write};

    println!();
    println!("  Select a version to install:");
    println!();
    println!("    1) latest   - newest stable release");
    println!("    2) nightly  - latest build from the dev branch");
    println!("    3) custom   - pin a specific version (e.g. v2.0.0)");
    println!();
    print!("  Choose [1]: ");
    io::stdout().flush().ok();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).ok();
    let choice = choice.trim();

    match choice {
        "2" => "nightly".to_string(),
        "3" => {
            print!("  Enter version (e.g. v2.0.0): ");
            io::stdout().flush().ok();
            let mut ver = String::new();
            io::stdin().read_line(&mut ver).ok();
            let ver = ver.trim().to_string();
            if ver.is_empty() { "latest".to_string() } else { ver }
        }
        _ => "latest".to_string(),
    }
}

fn select_variants(variant_arg: Option<String>, no_tui: bool) -> Vec<rl_manager::Variant> {
    let all = rl_manager::variants::all_variants();

    if let Some(arg) = variant_arg {
        if arg.trim() == "all" {
            return all;
        }
        return arg.split(',')
            .map(|s| s.trim().to_string())
            .filter_map(|name| all.iter().find(|v| v.name == name).cloned())
            .collect();
    }

    if let Ok(env_var) = std::env::var("RL_VARIANT") {
        if env_var.trim() == "all" {
            return all;
        }
        return env_var.split(',')
            .map(|s| s.trim().to_string())
            .filter_map(|name| all.iter().find(|v| v.name == name).cloned())
            .collect();
    }

    if no_tui || !std::io::stdin().is_terminal() {
        eprintln!("error: no TTY detected and RL_VARIANT is not set.");
        eprintln!("       Non-interactive use requires: RL_VARIANT=rl,rl_vm rlm install");
        std::process::exit(1);
    }

    // Interactive TUI picker
    pick_variants_interactive(&all)
}

fn pick_variants_interactive(all: &[rl_manager::Variant]) -> Vec<rl_manager::Variant> {
    use std::io::{self, Write};

    println!("  Select a build to install:");

    let mut current_group = "";
    for (i, variant) in all.iter().enumerate() {
        if variant.group != current_group {
            current_group = variant.group;
            println!();
            println!("  {}", current_group);
        }
        println!("    {:2}) {:24} ({})", i + 1, variant.name, variant.actual);
    }

    println!();
    println!("  Enter number(s), comma-separated (e.g. 1,3,9), or 'all'.");
    print!("  ");
    io::stdout().flush().ok();

    let mut choices = String::new();
    io::stdin().read_line(&mut choices).ok();
    let choices = choices.trim();

    if choices == "all" {
        return all.to_vec();
    }

    choices.split(',')
        .filter_map(|s| {
            let s = s.trim();
            let idx: usize = s.parse().ok()?;
            if idx >= 1 && idx <= all.len() {
                Some(all[idx - 1].clone())
            } else {
                eprintln!("  Invalid selection: {}", s);
                None
            }
        })
        .collect()
}

trait IsTerminal {
    fn is_terminal(&self) -> bool;
}

impl IsTerminal for std::io::Stdin {
    fn is_terminal(&self) -> bool {
        #[cfg(unix)]
        {
            unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
        }
        #[cfg(not(unix))]
        {
            true
        }
    }
}
