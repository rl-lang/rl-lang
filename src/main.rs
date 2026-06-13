use clap::{Parser, Subcommand};
use std::path::PathBuf;

use rl_lang::logic_loops::{eval_loop, lexing_loop, parsing_loop};
#[cfg(feature = "lsp")]
use rl_lang::lsp::run_lsp;
#[cfg(feature = "repl_tui")]
use rl_lang::repl;
#[cfg(feature = "repl_terminal")]
use rl_lang::repl_terminal;
use rl_lang::utils::source::SourceFile;

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
