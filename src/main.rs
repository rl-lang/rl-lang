//! CLI entry point for the `rl` command.
//!
//! Parses subcommands via [`clap`] and dispatches to the appropriate pipeline
//! functions or subsystems.
//!
//! | Subcommand | Action |
//! |---|---|
//! | `run <file>` | lex -> parse -> eval a `.rl` file |
//! | `dev` | read `rl.toml`, lex -> parse -> eval the project entry |
//! | `check <file>` | lex -> parse -> type-check, report errors |
//! | `new <name>` | scaffold a new project directory |
//! | `docs [topic]` | print stdlib / concept / tutorial reference |
//! | `repl` | start the interactive TUI REPL (`repl_tui` feature) |
//! | `lsp` | start the LSP server over stdio (`lsp` feature) |
use clap::{Parser, Subcommand};
use rl_lang::docs;
use rl_lang::tooling::new::create_project;
use rl_lang::tooling::package::{find_embedded, package};
use std::path::PathBuf;

#[cfg(feature = "lsp")]
use rl_lang::lsp::run_lsp;
#[cfg(feature = "repl_tui")]
use rl_lang::repl;
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
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
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

    /// Package a .rl file into a self-contained binary
    Package {
        /// Path to the .rl source file
        file: PathBuf,
        /// Output binary path
        #[arg(short, long, default_value = "program")]
        output: String,
    },
}

fn main() {
    // expriemental
    if let Some(source) = find_embedded() {
        let sf = SourceFile::new("program", source);
        let tokens = lexing_loop(sf.clone());
        let statements = parsing_loop(sf.clone(), tokens);
        if cfg!(feature = "eval") {
            eval_loop(sf, statements, 1);
        }
        return;
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, .. } => {
            let path = file
                .to_str()
                .unwrap_or_else(|| {
                    eprintln!("error: invalid file path");
                    std::process::exit(1);
                })
                .to_string();
            let source_text = std::fs::read_to_string(&file).unwrap_or_else(|_| {
                eprintln!("error: could not read file '{}'", file.display());
                std::process::exit(1);
            });
            let source = SourceFile::new(&*path, source_text);
            let tokens = lexing_loop(source.clone());
            let statements = parsing_loop(source.clone(), tokens);
            if cfg!(feature = "eval") {
                eval_loop(source, statements, 3);
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
                eval_loop(source, statements, 3);
            }
        }

        Commands::Check { file } => {
            let path = file
                .to_str()
                .unwrap_or_else(|| {
                    eprintln!("error: invalid file path");
                    std::process::exit(1);
                })
                .to_string();
            let source_text = std::fs::read_to_string(&file).unwrap_or_else(|_| {
                eprintln!("error: could not read file '{}'", file.display());
                std::process::exit(1);
            });
            let source = SourceFile::new(&*path, source_text);
            let tokens = lexing_loop(source.clone());
            let statements = parsing_loop(source.clone(), tokens);

            #[cfg(feature = "eval")]
            {
                use rl_lang::checker::TypeChecker;
                let mut checker = TypeChecker::new().with_source_file(source);
                let errors = checker.check(&statements);
                if errors.is_empty() {
                    println!("ok");
                } else {
                    for e in errors {
                        e.report_to_stderr();
                    }
                    std::process::exit(1);
                }
            }

            #[cfg(not(feature = "eval"))]
            println!("ok");
        }

        Commands::New { name } => {
            create_project(&name);
        }

        Commands::Docs { topic } => {
            let std_entries = docs::entries::stdlib_entries();
            let concept_entries = docs::entries::concept_entries();
            let tutorial_entries = docs::entries::tutorial_entries();

            match topic.as_deref() {
                None => {
                    println!("{}", docs::std_to_markdown(&std_entries));
                    println!("{}", docs::concept_to_markdown(&concept_entries));
                    println!("{}", docs::tutorial_to_markdown(&tutorial_entries));
                }
                Some(query) => {
                    // search stdlib entries
                    let matched_std: Vec<&docs::entry::StdEntry> = std_entries
                        .iter()
                        .copied()
                        .filter(|e| e.name.contains(query))
                        .collect();

                    // search concept entries
                    let matched_concepts: Vec<&docs::entry::ConceptEntry> = concept_entries
                        .iter()
                        .copied()
                        .filter(|e| e.name.contains(query))
                        .collect();

                    // search tutorial entries
                    let matched_tutorial: Vec<&docs::entry::ConceptEntry> = tutorial_entries
                        .iter()
                        .copied()
                        .filter(|e| e.name.contains(query))
                        .collect();

                    if matched_std.is_empty()
                        && matched_concepts.is_empty()
                        && matched_tutorial.is_empty()
                    {
                        eprintln!("no docs found for '{}'", query);
                        std::process::exit(1);
                    }

                    if !matched_std.is_empty() {
                        println!("{}", docs::std_to_markdown(&matched_std));
                    }
                    if !matched_concepts.is_empty() {
                        println!("{}", docs::concept_to_markdown(&matched_concepts));
                    }
                    if !matched_tutorial.is_empty() {
                        println!("{}", docs::tutorial_to_markdown(&matched_tutorial));
                    }
                }
            }
        }

        Commands::Repl => {
            #[cfg(feature = "repl_tui")]
            repl::start_repl();
        }

        #[cfg(feature = "lsp")]
        Commands::Lsp => match tokio::runtime::Runtime::new() {
            Ok(rt) => rt.block_on(run_lsp()),
            Err(e) => {
                eprintln!("error: failed to start LSP runtime: {}", e);
                std::process::exit(1);
            }
        },

        Commands::Package { file, output } => {
            let path = file.to_str().unwrap_or_else(|| {
                eprintln!("error: invalid file path");
                std::process::exit(1);
            });
            package(path, &output);
        }
    }
}
