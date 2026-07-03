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

// ---- macro start ----

#[macro_export]
macro_rules! assert_decl {
    (
        $source:expr,
        $variant:path,
        name: $name:expr,
        type_annotation: $ty:expr,
        value: $expr_kind:expr, $expr_span:expr,
        span: $stmt_span:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            $variant {
                name,
                type_annotation,
                value,
            } => {
                assert_eq!(name, $name);
                assert_eq!(*type_annotation, $ty);
                assert_eq!(ast.exprs.get(*value).kind, $expr_kind);
                assert_eq!(ast.exprs.get(*value).span, $expr_span);
            }
            other => panic!("expected {}, got {:?}", stringify!($variant), other),
        }
        assert_eq!(statements[0].span, $stmt_span);
    }};
}

#[macro_export]
macro_rules! assert_stmt {
    ($source:expr, $expected_kind:expr, $stmt_span:expr $(,)?) => {{
        let (_ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        assert_eq!(statements[0].kind, $expected_kind);
        assert_eq!(statements[0].span, $stmt_span);
    }};
}
// ---- macro end   ----
