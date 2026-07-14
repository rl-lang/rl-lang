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
mod logic_loops;
use clap::{Parser, Subcommand};
use rl_docs::{
    concept_to_markdown, docs_to_json,
    entries::{concept_entries, stdlib_entries, tutorial_entries},
    entry::{ConceptEntry, StdEntry},
    std_to_markdown, tutorial_to_markdown,
};
use rl_tooling::format::format_tokens;
use rl_tooling::new::create_project;
use rl_tooling::package::{find_embedded, package};
use rl_tooling::workflows::generate;
use std::path::PathBuf;

use crate::logic_loops::{eval_loop, lexing_loop, parsing_loop};
#[cfg(feature = "lsp")]
use rl_lsp::run_lsp;
use rl_tooling::dev::read_rl_toml;
use rl_utils::source::SourceFile;

#[derive(Parser)]
#[command(name = "rl", version, about = "The rl programming language")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .rl source file
    #[command(
        long_about = "Lex, parse, and evaluate a single .rl source file.\n\n\
                       Any arguments after the file path are passed through to the \
                       script as its argv, so flags meant for `rl` itself must come \
                       before the file path.",
        after_help = "EXAMPLES:\n    \
                       rl run script.rl\n    \
                       rl run script.rl -- --verbose input.txt"
    )]
    Run {
        /// Path to the .rl file to run
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Run thought the bytecode VM instead of the tree-walking evaluator
        /// (this is highly experimental)
        #[arg(long)]
        vm: bool,

        /// JIT compile via cranelift instead
        /// (this is very very highly experimental)
        #[arg(long)]
        cranelift: bool,

        /// Arguments forwarded to the script (accessible as argv inside .rl)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },

    /// Start the interactive TUI REPL
    #[command(long_about = "Start an interactive read-eval-print loop with syntax \
                             highlighting and history, running in the terminal.")]
    Repl,

    /// Run the current project (reads rl.toml)
    #[command(
        long_about = "Read `rl.toml` in the current directory and run the project's \
                       configured entry file.\n\n\
                       Use `rl new` first if you don't have an rl.toml yet."
    )]
    Dev,

    /// Scaffold a new project directory
    #[command(after_help = "EXAMPLES:\n    rl new my_project\n    rl new my_project --no-git")]
    New {
        /// Name for the new project directory
        #[arg(value_name = "NAME")]
        name: String,

        /// Skip running `git init` in the new project
        #[arg(long)]
        no_git: bool,
    },

    /// Type-check a .rl file and report errors without running it
    #[command(after_help = "EXAMPLES:\n    rl check script.rl")]
    Check {
        /// Path to the .rl file to check
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Scaffold GitHub Actions workflow files
    #[command(
        long_about = "Generate GitHub Actions workflow YAML for this project.\n\n\
                       At least one of --check or --package must be given.",
        after_help = "EXAMPLES:\n    \
                       rl workflows --check\n    \
                       rl workflows --package\n    \
                       rl workflows --check --package"
    )]
    Workflows {
        /// Generate a workflow that runs `rl check` on push/PR
        #[arg(long)]
        check: bool,

        /// Generate a workflow that packages and releases a binary
        #[arg(long)]
        package: bool,
    },

    /// Print language reference and stdlib documentation
    #[command(
        long_about = "Print rl's language reference, stdlib docs, and tutorials.\n\n\
                       With no TOPIC, prints everything. With a TOPIC, searches names \
                       across all categories for a match (narrow with --stdlib, \
                       --concept, or --tutorial).",
        after_help = "EXAMPLES:\n    \
                       rl docs\n    \
                       rl docs print\n    \
                       rl docs io --stdlib\n    \
                       rl docs loops --concept --json\n    \
                       rl docs --output docs.md"
    )]
    Docs {
        /// Name to search for (matches stdlib functions, concepts, or tutorial steps)
        #[arg(value_name = "TOPIC")]
        topic: Option<String>,

        /// Print output as JSON instead of Markdown
        #[arg(long)]
        json: bool,

        /// Restrict search to stdlib docs
        #[arg(long)]
        stdlib: bool,

        /// Restrict search to concept docs
        #[arg(long)]
        concept: bool,

        /// Restrict search to tutorial docs
        #[arg(long)]
        tutorial: bool,

        /// Write output to a file instead of stdout
        #[arg(long)]
        output: bool,

        /// Custom path for --output (implies --output)
        #[arg(long, value_name = "PATH")]
        out_file: Option<PathBuf>,

        /// Generate docs for current project
        #[arg(long)]
        generate: bool,
    },

    /// Start the LSP server over stdio
    #[cfg(feature = "lsp")]
    #[command(
        long_about = "Start the Language Server Protocol server, communicating \
                             over stdio. Intended to be launched by an editor, not run \
                             directly by hand."
    )]
    Lsp,

    /// Package a .rl file into a self-contained binary
    #[command(after_help = "EXAMPLES:\n    \
                             rl package script.rl\n    \
                             rl package script.rl --output myprogram")]
    Package {
        /// Path to the .rl source file to package
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output binary path
        #[arg(short, long, value_name = "PATH", default_value = "program")]
        output: String,
    },

    Format {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

fn main() {
    #[cfg(feature = "debug")]
    env_logger::init();

    // expriemental
    if let Some(source) = find_embedded() {
        let sf = SourceFile::new("program", source);
        let tokens = lexing_loop(sf.clone());
        let (ast, statements) = parsing_loop(sf.clone(), tokens);
        if cfg!(feature = "eval") {
            eval_loop(sf, ast, statements, 1);
        }
        return;
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            vm,
            cranelift,
            ..
        } => {
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
            let (ast, statements) = parsing_loop(source.clone(), tokens);
            // temporary solution
            // should be updated when:
            // *_loop accept refernce instead
            // `vm` and `cranelift` get separate compile command so it wouldn't
            // need the recompilation on every run step
            // thus no check every time
            #[cfg(feature = "eval")]
            {
                let tokens = lexing_loop(source.clone());
                let (checker_ast, checker_statements) = parsing_loop(source.clone(), tokens);
                use rl_checker::TypeChecker;
                let base_dir = file
                    .parent()
                    .map(std::path::Path::to_path_buf)
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let mut checker = TypeChecker::new()
                    .with_source_file(source.clone())
                    .with_ast_arena(checker_ast)
                    .with_base_dir(base_dir);
                let errors = checker.check(&checker_statements);
                if !errors.is_empty() {
                    for e in errors {
                        e.report_to_stderr();
                    }
                    std::process::exit(1);
                }
            }
            if vm {
                #[cfg(all(feature = "eval", feature = "vm"))]
                crate::logic_loops::vm_loop(source, ast, statements);
                #[cfg(not(feature = "vm"))]
                {
                    eprintln!("error: --vm requires the `vm` feature");
                    std::process::exit(1)
                }
                #[cfg(not(feature = "eval"))]
                {
                    eprintln!("error: --vm requires the `eval` feature");
                    std::process::exit(1)
                }
            } else if cranelift {
                #[cfg(all(feature = "cranelift", feature = "vm", feature = "eval"))]
                crate::logic_loops::cranelift_loop(source, ast, statements);
                #[cfg(not(all(feature = "eval", feature = "cranelift", feature = "vm")))]
                {
                    eprintln!("error: --cranelift requires the vm eval and cranelift features");
                    std::process::exit(1)
                }
            } else if cfg!(feature = "eval") {
                eval_loop(source, ast, statements, 3);
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
            let (ast, statements) = parsing_loop(source.clone(), tokens);
            if cfg!(feature = "eval") {
                eval_loop(source, ast, statements, 3);
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
            let (ast, statements) = parsing_loop(source.clone(), tokens);

            #[cfg(feature = "eval")]
            {
                use rl_checker::TypeChecker;
                let base_dir = file
                    .parent()
                    .map(std::path::Path::to_path_buf)
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let mut checker = TypeChecker::new()
                    .with_source_file(source)
                    .with_ast_arena(ast)
                    .with_base_dir(base_dir);
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

        Commands::Workflows { check, package } => {
            if !check && !package {
                eprintln!("error: specify at least --check or --package");
                std::process::exit(1);
            }
            generate(check, package);
        }

        Commands::New { name, no_git } => {
            create_project(&name, no_git);
        }

        Commands::Docs {
            topic,
            json,
            concept,
            tutorial,
            stdlib,
            output,
            out_file,
            generate,
        } => {
            if generate {
                #[cfg(feature = "eval")]
                {
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
                    let (ast, statements) = parsing_loop(source.clone(), tokens);

                    use rl_checker::TypeChecker;
                    let mut checker = TypeChecker::new()
                        .with_source_file(source)
                        .with_ast_arena(ast)
                        .with_base_dir(
                            path.parent()
                                .unwrap_or_else(|| std::path::Path::new("."))
                                .to_path_buf(),
                        );
                    let errors = checker.check(&statements);
                    if errors.is_empty() {
                        println!("check complete");
                    } else {
                        for e in errors {
                            e.report_to_stderr();
                        }
                        std::process::exit(1);
                    }

                    let parent = match path.parent() {
                        Some(p) => p,
                        None => {
                            eprintln!("error when formatting");
                            std::process::exit(1);
                        }
                    };

                    // format every .rl file in the project
                    let entries = match std::fs::read_dir(parent) {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("error when formatting: {}", e);
                            std::process::exit(1);
                        }
                    };
                    for entry in entries {
                        let entry = match entry {
                            Ok(e) => e,
                            Err(e) => {
                                eprintln!("error: {}", e);
                                continue;
                            }
                        };
                        let file_path = entry.path();
                        if file_path.extension().and_then(|e| e.to_str()) != Some("rl") {
                            continue;
                        }
                        let source_text =
                            std::fs::read_to_string(&file_path).unwrap_or_else(|_| {
                                eprintln!("error: could not read file '{}'", file_path.display());
                                std::process::exit(1);
                            });
                        let source = SourceFile::new(&*file_path.to_string_lossy(), source_text);
                        let tokens = lexing_loop(source);
                        let formatted = format_tokens(&tokens);
                        if let Err(e) = std::fs::write(&file_path, formatted) {
                            eprintln!("error: {}", e);
                        }
                    }

                    // generate website from /// doc comments
                    use rl_tooling::generate_docs::{extract_doc_items, write_doc_site};
                    let entries = match std::fs::read_dir(parent) {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("error when generating website: {}", e);
                            std::process::exit(1);
                        }
                    };
                    let mut all_items = Vec::new();
                    for entry in entries {
                        let entry = match entry {
                            Ok(e) => e,
                            Err(e) => {
                                eprintln!("error: {}", e);
                                continue;
                            }
                        };
                        let file_path = entry.path();
                        if file_path.extension().and_then(|e| e.to_str()) != Some("rl") {
                            continue;
                        }
                        let source_text =
                            std::fs::read_to_string(&file_path).unwrap_or_else(|_| {
                                eprintln!("error: could not read file '{}'", file_path.display());
                                std::process::exit(1);
                            });
                        let source = SourceFile::new(&*file_path.to_string_lossy(), source_text);
                        let tokens = lexing_loop(source);
                        let file_name = file_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown.rl")
                            .to_string();
                        all_items.extend(extract_doc_items(&tokens, &file_name));
                    }

                    let p = match parent.parent() {
                        Some(p) => p,
                        None => parent,
                    };
                    let doc_out_dir = p.join("docs_site");
                    if let Err(e) = write_doc_site(&all_items, &doc_out_dir, &config.project.name) {
                        eprintln!("error: failed to write doc site: {}", e);
                        std::process::exit(1);
                    }
                    println!(
                        "doc site written to '{}' ({} items)",
                        doc_out_dir.display(),
                        all_items.len()
                    );
                }
                return;
            }

            let std_entries = stdlib_entries();
            let concept_entries = concept_entries();
            let tutorial_entries = tutorial_entries();

            let any_category = stdlib || concept || tutorial;
            let want_std = !any_category || stdlib;
            let want_concept = !any_category || concept;
            let want_tutorial = !any_category || tutorial;

            let (matched_std, matched_concepts, matched_tutorial): (
                Vec<&StdEntry>,
                Vec<&ConceptEntry>,
                Vec<&ConceptEntry>,
            ) = match topic.as_deref() {
                None => (
                    if want_std {
                        std_entries.to_vec()
                    } else {
                        Vec::new()
                    },
                    if want_concept {
                        concept_entries.to_vec()
                    } else {
                        Vec::new()
                    },
                    if want_tutorial {
                        tutorial_entries.to_vec()
                    } else {
                        Vec::new()
                    },
                ),
                Some(query) => {
                    let matched_std = if want_std {
                        std_entries
                            .iter()
                            .copied()
                            .filter(|e| e.name.contains(query))
                            .collect()
                    } else {
                        Vec::new()
                    };

                    let matched_concepts = if want_concept {
                        concept_entries
                            .iter()
                            .copied()
                            .filter(|e| e.name.contains(query))
                            .collect()
                    } else {
                        Vec::new()
                    };

                    let matched_tutorial = if want_tutorial {
                        tutorial_entries
                            .iter()
                            .copied()
                            .filter(|e| e.name.contains(query))
                            .collect()
                    } else {
                        Vec::new()
                    };

                    if matched_std.is_empty()
                        && matched_concepts.is_empty()
                        && matched_tutorial.is_empty()
                    {
                        eprintln!("no docs found for '{}'", query);
                        std::process::exit(1);
                    }

                    (matched_std, matched_concepts, matched_tutorial)
                }
            };

            let rendered = if json {
                match docs_to_json(&matched_std, &matched_concepts, &matched_tutorial) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("error: failed to serialize docs to json: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                let mut out = String::new();
                if !matched_std.is_empty() {
                    out.push_str(&std_to_markdown(&matched_std));
                }
                if !matched_concepts.is_empty() {
                    out.push_str(&concept_to_markdown(&matched_concepts));
                }
                if !matched_tutorial.is_empty() {
                    out.push_str(&tutorial_to_markdown(&matched_tutorial));
                }
                out
            };

            let write_to_file = output || out_file.is_some();

            if write_to_file {
                let ext = if json { "json" } else { "md" };
                let filename =
                    out_file.unwrap_or_else(|| PathBuf::from(format!("docs_output.{}", ext)));
                if let Err(e) = std::fs::write(&filename, &rendered) {
                    eprintln!("error: failed to write '{}': {}", filename.display(), e);
                    std::process::exit(1);
                }
                println!("docs written to '{}'", filename.display());
            } else {
                println!("{}", rendered);
            }
        }

        Commands::Repl => {
            #[cfg(feature = "repl")]
            rl_repl::start_repl();
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

        Commands::Format { file } => {
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

            let tokens = lexing_loop(source);
            let formatted = format_tokens(&tokens);
            if let Err(e) = std::fs::write(path, formatted) {
                eprintln!("error: {}", e);
            };
        }
    }
}
