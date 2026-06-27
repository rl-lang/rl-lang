use crate::interpreter::stdlib::common::{check_arity, extract_string};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser_logic::Parser;
use crate::utils::{errors::Error, source::SourceFile, span::Span};

pub fn func(_: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "eval_isolated", span)?;

    let code = extract_string(args[0].clone(), "eval", span)?;

    let source = SourceFile::new("<eval>", code);

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => return Ok(Value::Err(Box::new(Value::String(e.message().to_string())))),
    };

    let ast = match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => return Ok(Value::Err(Box::new(Value::String(e.message().to_string())))),
    };

    let mut fresh_eval = Evaluator::default().with_stdlib().with_source_file(source);
    let resolved = fresh_eval.resolver.resolve_statements(ast);

    fresh_eval.output_buffer = Some(String::new());

    let result = fresh_eval.evaluate_block(&resolved);
    let captured = fresh_eval.output_buffer.take().unwrap_or_default();

    match result {
        Ok(_) => Ok(Value::Ok(Box::new(Value::String(captured)))),
        Err(e) => Ok(Value::Err(Box::new(Value::String(e.message().to_string())))),
    }
}
