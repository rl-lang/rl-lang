use crate::{
    interpreter::evaluator::Evaluator, lexer::tokenizer::Tokenizer,
    lsp::to_diagnostic::error_to_diagnostic, parser::parser_logic::Parser,
    utils::source::SourceFile,
};
use tower_lsp::lsp_types::Diagnostic;

/// rereads the entiire source file that is open
/// by lexing parsing and evaluating
/// three phases uses error_to_diagnostic to extract any Error and
/// transform it diagnostic to display
pub fn run_pipeline(source: &str) -> Vec<Diagnostic> {
    let file = SourceFile::new("buffer", source.to_string());

    let tokens = match Tokenizer::lex(file.clone()) {
        Ok(t) => t,
        Err(e) => return vec![error_to_diagnostic(source, &e)],
    };

    let statements = match Parser::parse(tokens, file.clone()) {
        Ok(s) => s,
        Err(e) => return vec![error_to_diagnostic(source, &e)],
    };

    let mut evaluator = Evaluator::default().with_stdlib().with_source_file(file);
    for stmt in &statements {
        if let Err(e) = evaluator.evaluate_statement(stmt) {
            return vec![error_to_diagnostic(source, &e)];
        }
    }

    // no errors to display
    vec![]
}
