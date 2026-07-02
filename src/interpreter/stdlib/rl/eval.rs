use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{extract_string, verr, vok, vs},
    values::Value,
};
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser_logic::Parser;
use crate::utils::source::SourceFile;

pub fn func(eval: &mut Evaluator, value: Value) -> Value {
    let code = match extract_string(value, "eval") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e)),
    };

    let source = SourceFile::new("<eval>", code);

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let ast = match Parser::parse(tokens, source) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let (ast, stmts) = ast;
    let mut resolver = std::mem::take(&mut eval.resolver);
    let resolved = resolver.resolve_statements(stmts);
    eval.resolver = resolver;

    let prev_buffer = eval.output_buffer.take();
    eval.output_buffer = Some(String::new());
    eval.environment.push(vec![]);

    let result = eval.evaluate_block(&ast, &resolved);

    eval.environment.pop();
    let captured = eval.output_buffer.take().unwrap_or_default();
    eval.output_buffer = prev_buffer;

    match result {
        Ok(_) => vok!(vs!(captured)),
        Err(e) => verr!(vs!(e.message().to_string())),
    }
}
