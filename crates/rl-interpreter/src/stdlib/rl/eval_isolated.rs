use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_string, verr, vok, vs},
    values::Value,
};
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

pub fn func(_: &mut Evaluator, value: Value) -> Value {
    let code = match extract_string(value, "eval_isolated") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e)),
    };

    let source = SourceFile::new("<eval>", code);

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let (ast, statements) = match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let mut fresh_eval = Evaluator::default().with_stdlib().with_source_file(source);
    let resolved = fresh_eval.resolver.resolve_program(ast, statements);

    fresh_eval.output_buffer = Some(String::new());

    let result = fresh_eval.evaluate_block(&resolved);
    let captured = fresh_eval.output_buffer.take().unwrap_or_default();

    match result {
        Ok(_) => vok!(vs!(captured)),
        Err(e) => verr!(vs!(e.message().to_string())),
    }
}
