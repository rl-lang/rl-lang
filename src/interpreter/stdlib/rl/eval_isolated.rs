use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{extract_string, verr, vok, vs},
    values::Value,
};
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser_logic::Parser;
use crate::utils::source::SourceFile;

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

    let ast = match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let (ast, stmts) = ast;

    let mut resolver = crate::resolver::Resolver::default();
    resolver.current_dir = std::path::Path::new(source.name.as_ref())
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .to_path_buf();

    let (arena, resolved) = resolver.resolve(ast, stmts);

    let mut fresh_eval = Evaluator::default()
        .with_stdlib()
        .with_source_file(source)
        .with_arena(arena);

    fresh_eval.resolver = resolver;
    fresh_eval.output_buffer = Some(String::new());

    let result = fresh_eval.evaluate_block(&resolved);
    let captured = fresh_eval.output_buffer.take().unwrap_or_default();

    match result {
        Ok(_) => vok!(vs!(captured)),
        Err(e) => verr!(vs!(e.message().to_string())),
    }
}
