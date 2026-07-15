//! Pipeline driver functions.
//!
//! Thin wrappers around each pipeline stage that handle errors uniformly:
//! print to stderr and exit with code 1. Used by both `main.rs` and any
//! other binary entry points that need to run the full pipeline without
//! boilerplate error handling at the call site.
//!
//! | Function | Stage |
//! |---|---|
//! | [`lexing_loop`] | source -> [`Vec<Token>`] |
//! | [`parsing_loop`] | tokens -> [`Vec<Statement>`] |
//! | [`eval_loop`] | statements -> execution (`eval` feature only) |
//!
//! [`Vec<Token>`]: crate::lexer::tokentypes::Token
//! [`Vec<Statement>`]: crate::ast::statements::Statement
#[cfg(feature = "debug")]
use log::info;

use rl_ast::Ast;

#[cfg(feature = "eval")]
use rl_interpreter::evaluator::Evaluator;

use {
    rl_ast::statements::Statement,
    rl_lexer::{tokenizer::Tokenizer, tokentypes::Token},
    rl_parser::parser_logic::Parser,
    rl_utils::source::SourceFile,
};

/// Lexes `source` into a token stream, or prints the error and exits.
pub fn lexing_loop(source: SourceFile) -> Vec<Token> {
    #[cfg(feature = "debug")]
    info!("lexing the source file...");
    match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => {
            e.report_to_stderr();
            std::process::exit(1);
        }
    }
}

/// Parses `tokens` into an AST statement list, or prints the error and exits.
pub fn parsing_loop(source: SourceFile, tokens: Vec<Token>) -> (Ast, Vec<Statement>) {
    #[cfg(feature = "debug")]
    info!("parsing the tokens into ast tree...");
    match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => {
            e.report_to_stderr();
            std::process::exit(1);
        }
    }
}

/// Resolves and evaluates `statements`, or prints the error and exits.
///
/// Only available with the `eval` feature. Constructs a fresh [`Evaluator`]
/// with the stdlib loaded, runs the [`Resolver`] pass, then evaluates the program.
///
/// [`Resolver`]: crate::resolver
#[cfg(feature = "eval")]
pub fn eval_loop(
    source: SourceFile,
    ast: Ast,
    statements: Vec<Statement>,
    user_args_offset: usize,
) {
    #[cfg(feature = "debug")]
    info!("evaluating the ast tree...");
    let mut evaluator = Evaluator::default()
        .with_stdlib()
        .with_source_file(source.clone())
        .with_user_args_offset(user_args_offset);

    evaluator.resolver.current_dir = std::path::Path::new(source.name.as_ref())
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .to_path_buf();

    let statements = evaluator.resolver.resolve_program(ast, statements);
    if let Err(e) = evaluator.evaluate_program(&statements) {
        e.report_to_stderr();
        std::process::exit(1);
    }

    #[cfg(feature = "debug")]
    info!("evaluation done");
}

#[cfg(all(feature = "eval", feature = "vm"))]
pub fn vm_loop(source: SourceFile, ast: Ast, statements: Vec<Statement>) {
    let chunk = compile_to_chunk(source, ast, statements);
    run_chunk(&chunk);
}

/// Resolves and compiles `statements` down to a bytecode [`rl_vm::Chunk`],
/// or prints the error and exits. Shared by `vm_loop` and the `compile`
/// subcommand.
#[cfg(all(feature = "eval", feature = "vm"))]
pub fn compile_to_chunk(source: SourceFile, ast: Ast, statements: Vec<Statement>) -> rl_vm::Chunk {
    use rl_vm::Compiler;

    let mut evaluator = Evaluator::default()
        .with_stdlib()
        .with_source_file(source.clone());

    evaluator.resolver.current_dir = std::path::Path::new(source.name.as_ref())
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .to_path_buf();

    let statements = evaluator.resolver.resolve_program(ast, statements);

    match Compiler::new(&evaluator.resolver.ast_arena).compile(&statements) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("vm compile error: {}", e.0);
            std::process::exit(1);
        }
    }
}

/// Runs an already-compiled [`rl_vm::Chunk`] on a fresh [`rl_vm::Vm`], or
/// prints the error and exits. Used both for freshly compiled `.rl` code
/// and bytecode loaded from a `.rlc` file.
#[cfg(all(feature = "eval", feature = "vm"))]
pub fn run_chunk(chunk: &rl_vm::Chunk) {
    use rl_vm::Vm;

    let mut vm = Vm::new();
    match vm.run(chunk) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("vm runtime error: {}", e.0);
            std::process::exit(1);
        }
    }
}

/// Loads and runs a precompiled `.rlc` bytecode file, or prints the error
/// and exits.
#[cfg(all(feature = "eval", feature = "vm"))]
pub fn run_rlc_file(path: &std::path::Path) {
    use rl_vm::{deserialize_chunk, stdlib};

    let bytes = std::fs::read(path).unwrap_or_else(|_| {
        eprintln!("error: could not read file '{}'", path.display());
        std::process::exit(1);
    });

    let chunk = match deserialize_chunk(&bytes, &stdlib::root()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to load '{}': {}", path.display(), e);
            std::process::exit(1);
        }
    };

    run_chunk(&chunk);
}

#[cfg(all(feature = "eval", feature = "cranelift", feature = "vm"))]
pub fn cranelift_loop(source: SourceFile, ast: Ast, statements: Vec<Statement>) {
    use rl_vm::Compiler;

    let mut evaluator = Evaluator::default()
        .with_stdlib()
        .with_source_file(source.clone());

    evaluator.resolver.current_dir = std::path::Path::new(source.name.as_ref())
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .to_path_buf();

    let statements = evaluator.resolver.resolve_program(ast, statements);

    let chunk = match Compiler::new(&evaluator.resolver.ast_arena).compile(&statements) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("vm compile error: {}", e.0);
            std::process::exit(1);
        }
    };

    match rl_cranelift::run_chunk(&chunk) {
        Ok(val) => println!("{}", val),
        Err(e) => {
            eprintln!("cranelift error: {}", e.0);
            std::process::exit(1);
        }
    }
}
