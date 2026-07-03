use rl_lang::{
    ast::{Ast, statements::Statement},
    interpreter::evaluator::Evaluator,
    utils::{errors::Error, source::SourceFile},
};

pub fn lex(source: &str) -> Vec<rl_lang::lexer::tokentypes::Token> {
    let text = SourceFile::new("test", source.to_string());
    rl_lang::logic_loops::lexing_loop(text)
}

pub fn parse(source: &str) -> (Ast, Vec<Statement>) {
    rl_lang::logic_loops::parsing_loop(SourceFile::new("test", source.to_string()), lex(source))
}

pub fn eval_program(source: &str) -> Result<Evaluator, Error> {
    let file = SourceFile::new("test", source.to_string());
    let tokens = rl_lang::lexer::tokenizer::Tokenizer::lex(file.clone())?;
    let (ast, stmts) = rl_lang::parser::parser_logic::Parser::parse(tokens, file.clone())?;
    let mut evaluator = Evaluator::default().with_stdlib().with_source_file(file);
    let stmts = evaluator.resolver.resolve_program(ast, stmts);
    evaluator.evaluate_program(&stmts)?;
    Ok(evaluator)
}
