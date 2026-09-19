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
    name = "rlc",
    version,
    about = "rl-lang compiler and runner",
    styles = RL_STYLES,
    long_about = "Lean compiler and runner for rl-lang.\n\n\
                   Compiles .rl source to .rlc bytecode, or runs source and bytecode directly.\n\
                   Unlike `rl`, this binary focuses on compile+run without project management,\n\
                   REPL, or docs commands.",
    after_help = "EXAMPLES:\n    \
                   rlc run script.rl          # compile + run\n    \
                   rlc run script.rlc         # run pre-compiled bytecode\n    \
                   rlc compile script.rl      # compile to .rlc\n    \
                   rlc compile script.rl -o out.rlc"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a .rl source file to .rlc bytecode
    #[command(after_help = "EXAMPLES:\n    \
                            rlc compile script.rl\n    \
                            rlc compile script.rl --output out.rlc")]
    Compile {
        /// Path to the .rl file to compile
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output .rlc path (defaults to FILE with its extension changed to .rlc)
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },

    /// Run a .rl source file or .rlc bytecode
    #[command(
        long_about = "Compile and run a .rl source file, or run pre-compiled .rlc bytecode.\n\n\
                       For .rl files: lex -> parse -> type-check -> compile -> run\n\
                       For .rlc files: deserialize -> run",
        after_help = "EXAMPLES:\n    \
                       rlc run script.rl\n    \
                       rlc run script.rlc\n    \
                       rlc run script.rl -- --verbose input.txt"
    )]
    Run {
        /// Path to the .rl or .rlc file to run
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Arguments forwarded to the script (accessible as argv inside .rl)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { file, output } => compile(&file, output),
        Commands::Run { file, .. } => run(&file),
    }
}

fn compile(file: &PathBuf, output: Option<PathBuf>) {
    use rl_checker::TypeChecker;
    use rl_utils::line_index::LineIndex;
    use rl_utils::source::SourceFile;

    let path = file
        .to_str()
        .unwrap_or_else(|| {
            eprintln!("error: invalid file path");
            std::process::exit(1);
        })
        .to_string();
    let source_text = std::fs::read_to_string(file).unwrap_or_else(|_| {
        eprintln!("error: could not read file '{}'", file.display());
        std::process::exit(1);
    });
    let source = SourceFile::new(&*path, source_text);

    // lex + parse
    let tokens = rl_cli::pipeline::lex::lex(source.clone());
    let (ast, statements) = rl_cli::pipeline::parse::parse(source.clone(), tokens);

    // type-check (separate parse pass)
    let checker_tokens = rl_cli::pipeline::lex::lex(source.clone());
    let (checker_ast, checker_statements) = rl_cli::pipeline::parse::parse(source.clone(), checker_tokens);
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

    // compile to bytecode
    let line_index = LineIndex::new(source.name.clone(), source.text.as_str());
    let chunk = rl_cli::pipeline::vm::compile_to_chunk(source, ast, statements);
    let bytes = rl_vm::serialize_chunk(&chunk, Some(&line_index));

    // write output
    let out_path = output.unwrap_or_else(|| file.with_extension("rlc"));
    if let Err(e) = std::fs::write(&out_path, &bytes) {
        eprintln!("error: failed to write '{}': {}", out_path.display(), e);
        std::process::exit(1);
    }
    println!("compiled '{}' -> '{}'", file.display(), out_path.display());
}

fn run(file: &PathBuf) {
    let is_rlc = file.extension().and_then(|e| e.to_str()) == Some("rlc");

    if is_rlc {
        rl_cli::pipeline::vm::run_rlc_file(file);
    } else {
        use rl_checker::TypeChecker;
        use rl_utils::source::SourceFile;

        let path = file
            .to_str()
            .unwrap_or_else(|| {
                eprintln!("error: invalid file path");
                std::process::exit(1);
            })
            .to_string();
        let source_text = std::fs::read_to_string(file).unwrap_or_else(|_| {
            eprintln!("error: could not read file '{}'", file.display());
            std::process::exit(1);
        });
        let source = SourceFile::new(&*path, source_text);

        // lex + parse
        let tokens = rl_cli::pipeline::lex::lex(source.clone());
        let (ast, statements) = rl_cli::pipeline::parse::parse(source.clone(), tokens);

        // type-check
        let checker_tokens = rl_cli::pipeline::lex::lex(source.clone());
        let (checker_ast, checker_statements) = rl_cli::pipeline::parse::parse(source.clone(), checker_tokens);
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

        // compile + run
        rl_cli::pipeline::vm::vm_loop(source, ast, statements);
    }
}
