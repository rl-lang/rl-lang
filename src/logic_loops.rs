#[cfg(feature = "debug")]
use log::info;

#[cfg(feature = "eval")]
use super::interpreter::evaluator::Evaluator;

use super::{
    ast::statements::Statement,
    lexer::{tokenizer::Tokenizer, tokentypes::Token},
    parser::parser_logic::Parser,
    utils::source::SourceFile,
};

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

pub fn parsing_loop(source: SourceFile, tokens: Vec<Token>) -> Vec<Statement> {
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

#[cfg(feature = "eval")]
pub fn eval_loop(source: SourceFile, statements: Vec<Statement>) {
    #[cfg(feature = "debug")]
    info!("evaluating the ast tree...");
    let mut evaluator = Evaluator::default().with_stdlib().with_source_file(source);
    for statement in statements {
        if let Err(e) = evaluator.evaluate_statement(&statement) {
            e.report_to_stderr();
            std::process::exit(1);
        }
    }

    #[cfg(feature = "debug")]
    info!("evaluation done");
}
