use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{extract_string, verr, vok, vs},
    values::Value,
};
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser_logic::Parser;
use crate::utils::{source::SourceFile, span::Span};

pub fn func(eval: &mut Evaluator, value: Value) -> Value {
    let code = match extract_string(value, "check", Span::dummy()) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let source = SourceFile::new("<check>", code);

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let ast = match Parser::parse(tokens, source) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let mut resolver = std::mem::take(&mut eval.resolver);
    resolver.resolve_statements(ast);
    eval.resolver = resolver;

    vok!(Value::Null)
}
