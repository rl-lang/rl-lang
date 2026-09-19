use std::path::PathBuf;

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
    name = "rlt",
    version,
    about = "rl-lang transpiler (to C99)",
    styles = RL_STYLES,
    long_about = "Transpile a .rl source file to C99.\n\n\
                   The resulting .c file can be compiled with any C compiler (gcc, clang, etc.).\n\n\
                   Use --runtime to also emit rl_runtime.h and rl_runtime.c.\n\
                   Use --compile to invoke cc after transpiling.",
    after_help = "EXAMPLES:\n    \
                   rlt script.rl\n    \
                   rlt script.rl --output out.c\n    \
                   rlt script.rl --runtime\n    \
                   rlt script.rl --runtime --compile\n    \
                   rlt script.rl --runtime --compile --opt O3"
)]
struct Cli {
    /// Path to the .rl file to transpile
    #[arg(value_name = "FILE")]
    file: PathBuf,

    /// Output .c path (defaults to FILE with its extension changed to .c)
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Also emit rl_runtime.h and rl_runtime.c
    #[arg(long)]
    runtime: bool,

    /// After transpiling, invoke cc to compile the .c file
    #[arg(short = 'c', long)]
    compile: bool,

    /// Optimization level forwarded to cc (O0, O1, O2, O3, Os, Og).
    /// Defaults to O2 when --compile is used.
    #[arg(long, value_name = "LEVEL")]
    opt: Option<String>,

    /// Extra flags forwarded to cc (only used with --compile)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    cc_flags: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let embed_rt = cli.runtime || cli.compile;
    let c_path = rl_cli::pipeline::cc::transpile_loop(&cli.file, cli.output, embed_rt);

    if cli.compile {
        let opt_flag = match cli.opt.as_deref() {
            Some(level) => format!("-O{}", level),
            None => "-O2".to_string(),
        };

        let mut cmd = std::process::Command::new("cc");
        cmd.arg(&opt_flag);
        cmd.arg("-o").arg(c_path.with_extension(""));
        cmd.arg(&c_path);

        if embed_rt {
            let dir = c_path.parent().unwrap_or(std::path::Path::new("."));
            cmd.arg(dir.join("rl_runtime.c"));
            cmd.arg("-I").arg(dir);
        }

        // auto-detect std::c usage and add required flags
        if let Ok(c_src) = std::fs::read_to_string(&c_path) {
            if c_src.contains("rl_c_") {
                cmd.arg("-DRL_USE_LIBFFI");
                cmd.arg("-lffi");
                cmd.arg("-ldl");
            }
            if c_src.contains("rl_http_") && c_src.contains("RL_USE_CURL") {
                cmd.arg("-DRL_USE_CURL");
                cmd.arg("-lcurl");
            }
        }

        if embed_rt {
            cmd.arg("-lm");
        }

        for flag in &cli.cc_flags {
            cmd.arg(flag);
        }

        let status = cmd.status().unwrap_or_else(|e| {
            eprintln!("error: failed to run cc: {}", e);
            std::process::exit(1);
        });

        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }

        println!(
            "compiled '{}' -> '{}' ({})",
            cli.file.display(),
            c_path.with_extension("").display(),
            opt_flag
        );
    }
}
