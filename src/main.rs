use clap::{Parser, Subcommand};
use rl_lang::docs;
use rl_lang::tooling::new::create_project;
use std::path::PathBuf;

#[cfg(feature = "lsp")]
use rl_lang::lsp::run_lsp;
#[cfg(feature = "repl_tui")]
use rl_lang::repl;
#[cfg(feature = "repl_terminal")]
use rl_lang::repl_terminal;
use rl_lang::utils::source::SourceFile;
use rl_lang::{
    logic_loops::{eval_loop, lexing_loop, parsing_loop},
    tooling::dev::read_rl_toml,
};

#[derive(Parser)]
#[command(name = "rl", version, about = "The rl programming language")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .rl source file
    Run {
        /// Path to the .rl file
        file: PathBuf,
    },
    /// Start the REPL
    Repl,

    /// Run the project directory
    Dev,

    /// Create a new project directory
    New {
        /// Name for the project
        name: String,
    },

    /// Checks the file for errors
    Check {
        /// Path to the .rl file
        file: PathBuf,
    },

    /// Print language reference and stdlib documentation
    // will be useful for multi use hehehe
    Docs {
        /// Topic to show (WIP)
        topic: Option<String>,
    },

    /// Start the LSP server
    #[cfg(feature = "lsp")]
    Lsp,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let path = file.to_str().unwrap().to_string();
            let source_text = std::fs::read_to_string(&file).unwrap_or_else(|_| {
                eprintln!("error: could not read file '{}'", file.display());
                std::process::exit(1);
            });
            let source = SourceFile::new(&*path, source_text);
            let tokens = lexing_loop(source.clone());
            let statements = parsing_loop(source.clone(), tokens);
            if cfg!(feature = "eval") {
                eval_loop(source, statements);
            }
        }

        Commands::Dev => {
            let config = read_rl_toml();
            let path = std::path::PathBuf::from(&config.project.entry);
            let source_text = std::fs::read_to_string(&path).unwrap_or_else(|_| {
                eprintln!(
                    "error: could not read entry file '{}'",
                    config.project.entry
                );
                std::process::exit(1);
            });
            println!("[{}] v{}", config.project.name, config.project.version);
            let source = SourceFile::new(&*config.project.entry, source_text);
            let tokens = lexing_loop(source.clone());
            let statements = parsing_loop(source.clone(), tokens);
            if cfg!(feature = "eval") {
                eval_loop(source, statements);
            }
        }

        Commands::Check { file } => {
            let path = file.to_str().unwrap().to_string();
            let source_text = std::fs::read_to_string(&file).unwrap_or_else(|_| {
                eprintln!("error: could not read file '{}'", file.display());
                std::process::exit(1);
            });
            let source = SourceFile::new(&*path, source_text);
            let tokens = lexing_loop(source.clone());
            parsing_loop(source.clone(), tokens);
            println!("ok");
        }

        Commands::New { name } => {
            create_project(&name);
        }

        // will move stdlib helper from repl to docs/ as single source of truth
        // not because i am lazy... really...
        Commands::Docs { .. } => {
            let enteries = docs::entries::stdlib_entries();
            println!("{}", docs::std_to_markdown(&enteries))
        }

        Commands::Repl => {
            #[cfg(feature = "repl_tui")]
            repl::start_repl();
            #[cfg(feature = "repl_terminal")]
            repl_terminal::start_repl();
        }

        #[cfg(feature = "lsp")]
        Commands::Lsp => {
            tokio::runtime::Runtime::new().unwrap().block_on(run_lsp());
        }
    }
}
