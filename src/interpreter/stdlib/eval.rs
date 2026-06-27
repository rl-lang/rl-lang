use crate::interpreter::stdlib::common::{check_arity, extract_string};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser_logic::Parser;
use crate::utils::{errors::Error, source::SourceFile, span::Span};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "eval", span)?;

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

    let mut resolver = std::mem::take(&mut eval.resolver);
    let resolved = resolver.resolve_statements(ast);
    eval.resolver = resolver;

    let prev_buffer = eval.output_buffer.take();
    eval.output_buffer = Some(String::new());

    eval.environment.push(vec![]);

    let result = eval.evaluate_block(&resolved);

    eval.environment.pop();
    let captured = eval.output_buffer.take().unwrap_or_default();
    eval.output_buffer = prev_buffer;

    match result {
        Ok(_) => Ok(Value::Ok(Box::new(Value::String(captured)))),
        Err(e) => Ok(Value::Err(Box::new(Value::String(e.message().to_string())))),
    }
}
