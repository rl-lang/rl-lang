use crate::{
    interpreter::evaluator::Evaluator,
    lexer::tokenizer::Tokenizer,
    parser::parser_logic::Parser,
    repl::{lines_types::OutputLine, syntax_highlighting::highlight},
    utils::source::SourceFile,
};

pub fn eval_input(input: &str, evaluator: &mut Evaluator, output: &mut Vec<OutputLine>) -> bool {
    let source = SourceFile::new("<repl>", input.to_string());

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => {
            output.push(OutputLine::Error(format!("error: {}", e.message())));
            return false;
        }
    };

    let statements = match Parser::parse(tokens, source.clone()) {
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
        if let crate::ast::statements::StatementKind::Expression(expr) = &statement.kind {
            match evaluator.evaluate(expr) {
                Ok(val) => {
                    if !matches!(val, crate::interpreter::values::Value::Null) {
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
