use rl_lang::ast::statements::StatementKind;
use rl_lang::ast::{ExprId, nodes::ExpressionKind};
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

pub fn assert_expr(ast: &Ast, id: ExprId, kind: ExpressionKind, span: rl_lang::utils::span::Span) {
    let expr = ast.exprs.get(id);
    assert_eq!(expr.kind, kind);
    assert_eq!(expr.span, span);
}

/// Checks `id` is a `Grouping` wrapping `inner_kind`/`inner_span`, and that
/// the grouping expression itself spans `outer_span`.
pub fn assert_grouping(
    ast: &Ast,
    id: ExprId,
    inner_kind: ExpressionKind,
    inner_span: rl_lang::utils::span::Span,
    outer_span: rl_lang::utils::span::Span,
) {
    let expr = ast.exprs.get(id);
    assert_eq!(expr.span, outer_span);
    match &expr.kind {
        ExpressionKind::Grouping(inner_id) => assert_expr(ast, *inner_id, inner_kind, inner_span),
        other => panic!("expected Grouping, got {:?}", other),
    }
}

/// Checks `stmt` is a single bare-expression statement wrapping `kind`/`expr_span`.
pub fn assert_single_expr_stmt(
    stmt: &Statement,
    ast: &Ast,
    kind: ExpressionKind,
    expr_span: rl_lang::utils::span::Span,
    stmt_span: rl_lang::utils::span::Span,
) {
    assert_eq!(stmt.span, stmt_span);
    match &stmt.kind {
        StatementKind::Expression(id) => assert_expr(ast, *id, kind, expr_span),
        other => panic!("expected Expression statement, got {:?}", other),
    }
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

#[macro_export]
macro_rules! assert_while {
    (
        $source:expr,
        condition: $cond_kind:expr, $cond_span:expr, grouped: $group_span:expr,
        body_expr: $body_kind:expr, $body_expr_span:expr, $body_stmt_span:expr,
        span: $stmt_span:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            rl_lang::ast::statements::StatementKind::While { condition, body } => {
                common::assert_grouping(&ast, *condition, $cond_kind, $cond_span, $group_span);
                assert_eq!(body.len(), 1, "expected exactly one body statement");
                common::assert_single_expr_stmt(
                    &body[0],
                    &ast,
                    $body_kind,
                    $body_expr_span,
                    $body_stmt_span,
                );
            }
            other => panic!("expected While, got {:?}", other),
        }
        assert_eq!(statements[0].span, $stmt_span);
    }};
}
// ---- macro end   ----
