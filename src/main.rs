use log::{debug, info};
use rl_lang::{
    interpreter::evaluator::Evaluator,
    lexer::tokenizer::Tokenizer,
    parser::parser_logic::Parser,
    repl,
    utils::errors::{Error, ErrorReason, Reason},
};

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
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Pipe(Box::new(
            std::fs::File::create("log.txt").unwrap(),
        )))
        .init();
    info!("logger initialized");

    // arguments
    info!("reading arguments");
    let arguments: Vec<String> = std::env::args().collect();
    debug!("used arguments [{:?}]", arguments);
    if arguments.len() == 3 && arguments[1] != "run" {
        eprintln!("Usage: rlp run <source-file.rl>");
        return;
    } else if arguments.len() == 3 && arguments[1] == "run" {
        log::info!("starting the interpreter");
    } else if arguments.len() == 2 && arguments[1] == "repl" {
        log::info!("starting repl shell");
        println!("[Starting REPL shell]");
        repl::repl();
        return;
    } else {
        log::info!("falling back to repl");
        println!("[Starting REPL shell]");
        repl::repl();
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

    let source_file = match std::fs::read_to_string(&arguments[2]) {
        Ok(file) => file,

        Err(_) => {
            Error::init(
                "failed to read file".to_string(),
                None,
                Some(ErrorReason::init(Reason::Compile, None)),
            )
            .print_error();
            return;
        }
    };

    println!("[Parsing source file: {}]", arguments[2]);
    // phase one: lexing the source file into tokens
    info!("lexing the source file...");
    let tokens = Tokenizer::lex(&source_file);

    // phase two: parsing the tokens into ast tree
    info!("parsing the tokens into ast tree...");
    let statements = Parser::parse(tokens);

    // phase three: evaluating the ast tree
    info!("evaluating the ast tree...");
    let mut evaluator = Evaluator::default().with_stdlib();
    for statement in statements {
        evaluator.evaluate_statement(&statement);
    }
    info!("evaluation done")
}
