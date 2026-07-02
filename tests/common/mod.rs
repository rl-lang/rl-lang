use rl_lang::{
    // ast::StmtId,
    ast::{Ast, StmtId},
    interpreter::evaluator::Evaluator,
    utils::{errors::Error, source::SourceFile},
};

pub fn lex(source: &str) -> Vec<rl_lang::lexer::tokentypes::Token> {
    let text = SourceFile::new("test", source.to_string());
    rl_lang::logic_loops::lexing_loop(text)
}

pub fn parse(source: &str) -> (Ast, Vec<StmtId>) {
    rl_lang::logic_loops::parsing_loop(SourceFile::new("test", source.to_string()), lex(source))
}

pub fn eval_program(source: &str) -> Result<Evaluator, Error> {
    let file = SourceFile::new("test", source.to_string());
    let tokens = rl_lang::lexer::tokenizer::Tokenizer::lex(file.clone())?;

    let (ast, stmts) = rl_lang::parser::parser_logic::Parser::parse(tokens, file.clone())?;
    let mut evaluator = Evaluator::default()
        .with_stdlib()
        .with_source_file(file.clone());

    evaluator.resolver = evaluator.resolver.with_ast(ast);
    evaluator.resolver.current_dir = std::path::Path::new(file.name.as_ref())
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .to_path_buf();
    let resolved_stmts = evaluator.resolver.resolve_statements(stmts);
    let resolved_ast = evaluator.resolver.into_ast();

    evaluator.evaluate_program(&resolved_ast, &resolved_stmts)?;

    Ok(evaluator)
}
