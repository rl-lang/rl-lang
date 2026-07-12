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
    use rl_vm::{Compiler, Vm};

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

    println!("{:?}", chunk);

    let mut vm = Vm::new();
    match vm.run(&chunk) {
        Ok(Some(val)) => println!("{:?}", val),
        Ok(None) => {}
        Err(e) => {
            eprintln!("vm runtime error: {}", e.0);
            std::process::exit(1);
        }
    }
}
