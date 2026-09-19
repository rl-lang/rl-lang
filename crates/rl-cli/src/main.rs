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
use rl_cli::pipeline;
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
use rl_tooling::new::create_project;
use rl_tooling::package::{find_embedded, package};
use rl_tooling::workflows::generate;
use rl_tooling::{format::format_tokens, package::EmbeddedProgram};
use std::path::PathBuf;

use pipeline::lex::lex;
use pipeline::parse::parse;
use rl_tooling::dev::read_rl_toml;
use rl_utils::source::SourceFile;

#[derive(Parser)]
#[command(name = "rl", version, about = "The rl programming language", styles = RL_STYLES)]
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
        #[arg(value_name = "FILE", required_unless_present = "code")]
        file: Option<PathBuf>,

        /// Execute inline rl code instead of a file
        #[arg(short = 'c', long = "code", value_name = "CODE")]
        code: Option<String>,

        /// Run through the bytecode VM
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

    /// Run the current project (reads rl.toml)
    #[command(
        long_about = "Read `rl.toml` in the current directory and run the project's \
                       configured entry file.\n\n\
                       Use `rl new` first if you don't have an rl.toml yet.",
        after_help = "EXAMPLES:\n    \
                       rl dev\n    \
                       rl dev --vm\n    \
                       rl dev --cranelift"
    )]
    Dev {
        /// Run through the bytecode VM
        /// (this is highly experimental)
        #[arg(long)]
        vm: bool,

        /// JIT compile via cranelift instead
        /// (this is very very highly experimental)
        #[arg(long)]
        cranelift: bool,
    },

    /// Scaffold a new project directory, or create a standalone script
    #[command(after_help = "EXAMPLES:\n    rl new my_project\n    rl new my_project --no-git\n    rl new my_project --lib\n    rl new --script hello")]
    New {
        /// Name for the new project directory or script
        #[arg(value_name = "NAME")]
        name: String,

        /// Skip running `git init` in the new project
        #[arg(long)]
        no_git: bool,

        /// Create a standalone .rl script instead of a project directory
        #[arg(long)]
        script: bool,

        /// Create a library project (generates src/lib.rl instead of src/main.rl)
        #[arg(long)]
        lib: bool,
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

    /// Package a .rl file into a self-contained binary
    #[command(after_help = "EXAMPLES:\n    \
                                 rl package script.rl\n    \
                                 rl package script.rl --output myprogram\n    \
                                 rl package script.rl --vm")]
    Package {
        /// Path to the .rl source file to package
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output binary path
        #[arg(short, long, value_name = "PATH", default_value = "program")]
        output: String,

        /// Compile to .rlc bytecode first and embed that (compressed)
        /// instead of the raw source text. The resulting binary loads
        /// straight into the VM at startup, skipping lex/parse/compile.
        #[arg(long)]
        vm: bool,
    },

    Format {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Print debug info for a .rl source file (tokens, parser, or AST)
    #[command(
        long_about = "Print debug information for a single .rl source file.\n\n\
                       Exactly one of --tokens, --parser, or --ast must be given.",
        after_help = "EXAMPLES:\n    \
                       rl print script.rl --tokens\n    \
                       rl print script.rl --parser\n    \
                       rl print script.rl --ast"
    )]
    Print {
        /// Path to the .rl file to inspect
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Print the token stream
        #[arg(short = 't', long)]
        tokens: bool,

        /// Print the parsed AST (pre-type-check)
        #[arg(short = 'p', long)]
        parser: bool,

        /// Print the type-checked AST
        #[arg(short = 'a', long)]
        ast: bool,
    },

    /// Package manager for rl dependencies
    #[cfg(feature = "pm")]
    #[command(
        long_about = "Manage project dependencies.\n\n\
                       Downloads tarballs, verifies SHA256, and creates symlinks in deps/.",
        after_help = "EXAMPLES:\n    \
                       rl pm install\n    \
                       rl pm add csv https://example.com/csv-0.1.0.tar.gz\n    \
                       rl pm add csv https://example.com/csv-0.1.0.tar.gz --sha256 abc123\n    \
                       rl pm remove csv\n    \
                       rl pm list\n    \
                       rl pm update\n    \
                       rl pm cache clean"
    )]
    Pm {
        #[command(subcommand)]
        command: PmCommands,
    },
}

#[cfg(feature = "pm")]
#[derive(Subcommand)]
enum PmCommands {
    /// Install all dependencies from rl.toml
    Install,
    /// Add a dependency to rl.toml and download it
    Add {
        /// Package name
        #[arg(value_name = "NAME")]
        name: String,
        /// Tarball URL
        #[arg(value_name = "URL")]
        url: String,
        /// Expected SHA256 hash (optional)
        #[arg(long, value_name = "HASH")]
        sha256: Option<String>,
    },
    /// Remove a dependency from rl.toml and delete its symlink
    Remove {
        /// Package name
        #[arg(value_name = "NAME")]
        name: String,
    },
    /// List installed dependencies
    List,
    /// Re-download all dependencies and verify hashes
    Update,
    /// Manage the download cache
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },
}

#[cfg(feature = "pm")]
#[derive(Subcommand)]
enum CacheCommands {
    /// Clear the download cache
    Clean,
}

fn main() {
    #[cfg(feature = "debug")]
    env_logger::init();

    // expriemental
    match find_embedded() {
        Some(EmbeddedProgram::Source(source)) => {
            let sf = SourceFile::new("program", source);
            let tokens = lex(sf.clone());
            let (ast, statements) = parse(sf.clone(), tokens);
            #[cfg(feature = "vm")]
            {
                pipeline::vm::vm_loop(sf, ast, statements);
                return;
            }
            #[cfg(not(feature = "vm"))]
            {
                let _ = (sf, ast, statements);
                eprintln!("error: running source-packaged binaries requires the `vm` feature");
                std::process::exit(1);
            }
        }
        Some(EmbeddedProgram::Bytecode(bytes)) => {
            #[cfg(feature = "vm")]
            {
                use pipeline::vm::run_rlc_bytes;
                run_rlc_bytes(&bytes, "program");
                return;
            }
            #[cfg(not(feature = "vm"))]
            {
                let _ = bytes;
                eprintln!("error: running vm-packaged binaries requires the `vm` feature");
                std::process::exit(1);
            }
        }
        None => {}
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            code,
            vm,
            cranelift,
            ..
        } => {
            // Inline code mode: rl run -c "code here"
            if let Some(code_str) = code {
                let source = SourceFile::new("<eval>", code_str);
                let tokens = lex(source.clone());
                let (ast, statements) = parse(source.clone(), tokens);
                {
                    let tokens = lex(source.clone());
                    let (checker_ast, checker_statements) = parse(source.clone(), tokens);
                    use rl_checker::TypeChecker;
                    let mut checker = TypeChecker::new()
                        .with_source_file(source.clone())
                        .with_ast_arena(checker_ast)
                        .with_base_dir(std::path::PathBuf::from("."));
                    checker.check(&checker_statements);
                    for w in &checker.warnings {
                        w.report_to_stderr();
                    }
                    if !checker.errors.is_empty() {
                        for e in &checker.errors {
                            e.report_to_stderr();
                        }
                        std::process::exit(1);
                    }
                }
                if vm {
                    #[cfg(feature = "vm")]
                    pipeline::vm::vm_loop(source, ast, statements);
                    #[cfg(not(feature = "vm"))]
                    {
                        eprintln!("error: --vm requires the `vm` feature");
                        std::process::exit(1)
                    }
                } else if cranelift {
                    #[cfg(feature = "cranelift")]
                    pipeline::vm::cranelift_loop(source, ast, statements);
                    #[cfg(not(feature = "cranelift"))]
                    {
                        eprintln!("error: --cranelift requires the `cranelift` feature");
                        std::process::exit(1)
                    }
                } else {
                    #[cfg(feature = "vm")]
                    pipeline::vm::vm_loop(source, ast, statements);
                    #[cfg(not(feature = "vm"))]
                    {
                        eprintln!("error: this build of rl has no execution backend (missing the `vm` feature)");
                        std::process::exit(1);
                    }
                }
                return;
            }

            let file = file.unwrap_or_else(|| {
                eprintln!("error: either a file path or -c/--code is required");
                std::process::exit(1);
            });

            let is_rlc = file.extension().and_then(|e| e.to_str()) == Some("rlc");

            if is_rlc {
                #[cfg(feature = "vm")]
                {
                    use pipeline::vm::run_rlc_file;
                    run_rlc_file(&file);
                    return;
                }
                #[cfg(not(feature = "vm"))]
                {
                    eprintln!("error: running .rlc files requires the `vm` feature");
                    std::process::exit(1);
                }
            }

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
            let tokens = lex(source.clone());
            let (ast, statements) = parse(source.clone(), tokens);
            {
                let tokens = lex(source.clone());
                let (checker_ast, checker_statements) = parse(source.clone(), tokens);
                use rl_checker::TypeChecker;
                let base_dir = file
                    .parent()
                    .map(std::path::Path::to_path_buf)
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let mut checker = TypeChecker::new()
                    .with_source_file(source.clone())
                    .with_ast_arena(checker_ast)
                    .with_base_dir(base_dir);
                checker.check(&checker_statements);
                for w in &checker.warnings {
                    w.report_to_stderr();
                }
                if !checker.errors.is_empty() {
                    for e in &checker.errors {
                        e.report_to_stderr();
                    }
                    std::process::exit(1);
                }
            }
            if vm {
                #[cfg(feature = "vm")]
                pipeline::vm::vm_loop(source, ast, statements);
                #[cfg(not(feature = "vm"))]
                {
                    eprintln!("error: --vm requires the `vm` feature");
                    std::process::exit(1)
                }
            } else if cranelift {
                #[cfg(feature = "cranelift")]
                pipeline::vm::cranelift_loop(source, ast, statements);
                #[cfg(not(feature = "cranelift"))]
                {
                    eprintln!(
                        "error: --cranelift requires the `cranelift` feature (which implies `vm`)"
                    );
                    std::process::exit(1)
                }
            } else {
                #[cfg(feature = "vm")]
                pipeline::vm::vm_loop(source, ast, statements);
                #[cfg(not(feature = "vm"))]
                {
                    let _ = (&ast, &statements);
                    eprintln!(
                        "error: this build of rl has no execution backend (missing the `vm` feature)"
                    );
                    std::process::exit(1);
                }
            }
        }

        Commands::Dev { vm, cranelift } => {
            let config = read_rl_toml();

            // warn if [dependencies] section is missing
            let raw = std::fs::read_to_string("rl.toml").unwrap_or_default();
            if !rl_tooling::dev::has_dependencies_section(&raw) {
                eprintln!("warning: rl.toml is missing a [dependencies] section");
                eprintln!("  add an empty [dependencies] section to silence this warning");
            }

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
            let tokens = lex(source.clone());
            let (ast, statements) = parse(source.clone(), tokens);
            if vm {
                #[cfg(feature = "vm")]
                pipeline::vm::vm_loop(source, ast, statements);
                #[cfg(not(feature = "vm"))]
                {
                    eprintln!("error: --vm requires the `vm` feature");
                    std::process::exit(1)
                }
            } else if cranelift {
                #[cfg(feature = "cranelift")]
                pipeline::vm::cranelift_loop(source, ast, statements);
                #[cfg(not(feature = "cranelift"))]
                {
                    eprintln!(
                        "error: --cranelift requires the `cranelift` feature (which implies `vm`)"
                    );
                    std::process::exit(1)
                }
            } else {
                #[cfg(feature = "vm")]
                pipeline::vm::vm_loop(source, ast, statements);
                #[cfg(not(feature = "vm"))]
                {
                    let _ = (&ast, &statements);
                    eprintln!(
                        "error: this build of rl has no execution backend (missing the `vm` feature)"
                    );
                    std::process::exit(1);
                }
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
            let tokens = lex(source.clone());
            let (ast, statements) = parse(source.clone(), tokens);

            use rl_checker::TypeChecker;
            let base_dir = file
                .parent()
                .map(std::path::Path::to_path_buf)
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            let mut checker = TypeChecker::new()
                .with_source_file(source)
                .with_ast_arena(ast)
                .with_base_dir(base_dir);
            checker.check(&statements);
            for w in &checker.warnings {
                w.report_to_stderr();
            }
            if checker.errors.is_empty() {
                println!("ok");
            } else {
                for e in &checker.errors {
                    e.report_to_stderr();
                }
                std::process::exit(1);
            }
        }

        Commands::Workflows { check, package } => {
            if !check && !package {
                eprintln!("error: specify at least --check or --package");
                std::process::exit(1);
            }
            generate(check, package);
        }

        Commands::New { name, no_git, script, lib } => {
            if script {
                rl_tooling::new::create_script(&name);
            } else {
                create_project(&name, no_git, lib);
            }
        }

        #[cfg(feature = "docs")]
        Commands::Package { file, output, vm } => {
            let path = file.to_str().unwrap_or_else(|| {
                eprintln!("error: invalid file path");
                std::process::exit(1);
            });

            if vm {
                #[cfg(feature = "vm")]
                {
                    use pipeline::vm::compile_to_chunk;

                    let source_text = std::fs::read_to_string(&file).unwrap_or_else(|_| {
                        eprintln!("error: could not read file '{}'", file.display());
                        std::process::exit(1);
                    });
                    let source = SourceFile::new(path, source_text);
                    let tokens = lex(source.clone());
                    let (ast, statements) = parse(source.clone(), tokens);

                    let checker_tokens = lex(source.clone());
                    let (checker_ast, checker_statements) =
                        parse(source.clone(), checker_tokens);
                    use rl_checker::TypeChecker;
                    use rl_tooling::package::package_vm;
                    let base_dir = file
                        .parent()
                        .map(std::path::Path::to_path_buf)
                        .unwrap_or_else(|| std::path::PathBuf::from("."));
                    let mut checker = TypeChecker::new()
                        .with_source_file(source.clone())
                        .with_ast_arena(checker_ast)
                        .with_base_dir(base_dir);
                    checker.check(&checker_statements);
                    for w in &checker.warnings {
                        w.report_to_stderr();
                    }
                    if !checker.errors.is_empty() {
                        for e in &checker.errors {
                            e.report_to_stderr();
                        }
                        std::process::exit(1);
                    }

                    let line_index = rl_utils::line_index::LineIndex::new(
                        source.name.clone(),
                        source.text.as_str(),
                    );
                    let chunk = compile_to_chunk(source, ast, statements);
                    let bytecode = rl_vm::serialize_chunk(&chunk, Some(&line_index));
                    package_vm(&bytecode, &output);
                }
                #[cfg(not(feature = "vm"))]
                {
                    eprintln!("error: `package --vm` requires the `vm` feature");
                    std::process::exit(1);
                }
            } else {
                package(path, &output);
            }
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

            let tokens = lex(source);
            let formatted = format_tokens(&tokens);
            if let Err(e) = std::fs::write(path, formatted) {
                eprintln!("error: {}", e);
            };
        }

        Commands::Print { file, tokens, parser, ast } => {
            if !tokens && !parser && !ast {
                eprintln!("error: specify at least one of --tokens, --parser, or --ast");
                std::process::exit(1);
            }
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

            if tokens {
                let toks = lex(source.clone());
                rl_tooling::tree_print::print_tokens(&toks);
            }
            if parser {
                let toks = lex(source.clone());
                let (parsed_ast, statements) = parse(source.clone(), toks);
                rl_tooling::tree_print::print_statements(&statements, &parsed_ast.exprs, "Statements (parser)");
            }
            if ast {
                let toks = lex(source.clone());
                let (checker_ast, checker_statements) = parse(source.clone(), toks);
                use rl_checker::TypeChecker;
                let base_dir = file
                    .parent()
                    .map(std::path::Path::to_path_buf)
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let mut checker = TypeChecker::new()
                    .with_source_file(source.clone())
                    .with_ast_arena(checker_ast)
                    .with_base_dir(base_dir);
                checker.check(&checker_statements);
                for w in &checker.warnings {
                    w.report_to_stderr();
                }
                if !checker.errors.is_empty() {
                    for e in &checker.errors {
                        e.report_to_stderr();
                    }
                    std::process::exit(1);
                }
                rl_tooling::tree_print::print_statements(&checker_statements, &checker.ast_arena.exprs, "Statements (resolved)");
            }
        }

        #[cfg(feature = "pm")]
        Commands::Pm { command } => {
            let project_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            match command {
                PmCommands::Install => {
                    if let Err(e) = rl_pm::install_all(&project_root) {
                        eprintln!("error: {}", e);
                        std::process::exit(1);
                    }
                }
                PmCommands::Add { name, url, sha256 } => {
                    if let Err(e) = rl_pm::add_dep(&project_root, &name, &url, sha256.as_deref()) {
                        eprintln!("error: {}", e);
                        std::process::exit(1);
                    }
                }
                PmCommands::Remove { name } => {
                    if let Err(e) = rl_pm::remove_dep(&project_root, &name) {
                        eprintln!("error: {}", e);
                        std::process::exit(1);
                    }
                }
                PmCommands::List => {
                    match rl_pm::list_deps(&project_root) {
                        Ok(deps) => {
                            if deps.is_empty() {
                                eprintln!("no dependencies");
                            } else {
                                for dep in &deps {
                                    let status = if dep.installed { "installed" } else { "missing" };
                                    eprintln!("{} {} [{}]", dep.name, dep.url, status);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                PmCommands::Update => {
                    if let Err(e) = rl_pm::update_deps(&project_root) {
                        eprintln!("error: {}", e);
                        std::process::exit(1);
                    }
                }
                PmCommands::Cache { command } => match command {
                    CacheCommands::Clean => {
                        if let Err(e) = rl_pm::cache_clean() {
                            eprintln!("error: {}", e);
                            std::process::exit(1);
                        }
                    }
                },
            }
        }
    }
}
