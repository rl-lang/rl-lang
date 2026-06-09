#[cfg(feature = "debug")]
use log::{debug, info};

use rl_lang::logic_loops::{eval_loop, lexing_loop, parsing_loop, validate_source_arg};

use rl_lang::utils::{
    errors::{Error, ErrorReason, Reason},
    source::SourceFile,
};

#[cfg(feature = "repl_tui")]
use rl_lang::repl;

#[cfg(feature = "repl_terminal")]
use rl_lang::repl_terminal;

/// entry point for `rl` interpreter
///
/// # usage
/// ```bash
/// rl run <source.rl>
/// ```
///
/// # phases
/// 1. **lexing**
/// 2. **parsing**
/// 3. **evaluating**
fn main() {
    // initializing the logger
    #[cfg(feature = "debug")]
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Pipe(Box::new(
            std::fs::File::create("log.txt").unwrap(),
        )))
        .init();
    #[cfg(feature = "debug")]
    info!("logger initialized");

    // arguments
    #[cfg(feature = "debug")]
    info!("reading arguments");

    let arguments: Vec<String> = std::env::args().collect();

    #[cfg(feature = "debug")]
    debug!("used arguments [{:?}]", arguments);

    if cfg!(feature = "run") && arguments[1] == "run" {
        #[cfg(feature = "debug")]
        log::info!("starting the interpreter");
    } else if cfg!(feature = "repl") && arguments.len() == 2 && arguments[1] == "repl" {
        if cfg!(feature = "repl_tui") {
            #[cfg(feature = "debug")]
            log::info!("starting repl TUI");
            repl::start_repl();
        } else if cfg!(feature = "repl_terminal") {
            #[cfg(feature = "debug")]
            log::info!("starting repl terminal shell");
            println!("[Starting REPL shell]");
            #[cfg(feature = "repl_terminal")]
            repl_terminal::start_repl();
        }
        return;
    } else {
        return;
    }

    // check the source file if it ends with rl extension and then parse it to string
    if !arguments[2].ends_with(".rl") {
        Error::init(
            "file extension is not .rl".to_string(),
            None,
            Some(ErrorReason::init(Reason::Compile, None)),
        )
        .print_error();
        return;
    }

    let source_file = validate_source_arg(&arguments).unwrap_or_else(|_| std::process::exit(1));

    println!("[Parsing source file: {}]", arguments[2]);
    let source = SourceFile::new(arguments[2].as_str(), source_file);

    // phase one: lexing the source file into tokens
    let tokens = lexing_loop(source.clone());

    // phase two: parsing the tokens into ast tree
    let statements = parsing_loop(source.clone(), tokens);

    // phase three: evaluating the ast tree
    if cfg!(feature = "eval") {
        eval_loop(source, statements);
    }
}
