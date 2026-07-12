//! Evaluates a complete input string and appends results to the output buffer.
use rl_ast::statements::StatementKind;
use rl_interpreter::{evaluator::Evaluator, values::Value};
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

use crate::{lines_types::OutputLine, syntax_highlighting::highlight};

/// Lexes, parses, and evaluates `input`, appending results to `output`.
///
/// Expression statements have their return value rendered directly with syntax
/// highlighting (unless the value is `null`). Non-expression statements
/// (declarations, loops, etc.) are evaluated for their side effects only.
///
/// Any `println` / `print` output captured in [`Evaluator::output_buffer`]
/// is flushed into `output` as [`OutputLine::Result`] lines after evaluation.
///
/// Returns `true` if all statements evaluated without error.
pub fn eval_input(input: &str, evaluator: &mut Evaluator, output: &mut Vec<OutputLine>) -> bool {
    let source = SourceFile::new("<repl>", input.to_string());

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => {
            output.push(OutputLine::Error(format!("error: {}", e.message())));
            return false;
        }
    };

    let (_file_ast, statements) = match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => {
            output.push(OutputLine::Error(format!("error: {}", e.message())));
            return false;
        }
    };

    evaluator.set_source_file(source);
    evaluator.output_buffer = Some(String::new());

    let mut success = true;

    for statement in &statements {
        if let StatementKind::Expression(expr) = &statement.kind {
            match evaluator.evaluate(*expr) {
                Ok(val) => {
                    if !matches!(val, Value::Null) {
                        let val_str = format!("{}", val);
                        let spans = highlight(&val_str);
                        output.push(OutputLine::Styled(
                            spans
                                .into_iter()
                                .map(|sp| (sp.content.into_owned(), sp.style))
                                .collect(),
                        ));
                    }
                }
                Err(e) => {
                    output.push(OutputLine::Error(format!("error: {}", e.message())));
                    success = false;
                    break;
                }
            }
        } else if let Err(e) = evaluator.evaluate_statement(statement) {
            output.push(OutputLine::Error(format!("error: {}", e.message())));
            success = false;
            break;
        }
    }

    if let Some(captured) = evaluator.output_buffer.take() {
        for line in captured.split('\n') {
            if !line.is_empty() {
                output.push(OutputLine::Result(line.to_string()));
            }
        }
    }

    success
}
