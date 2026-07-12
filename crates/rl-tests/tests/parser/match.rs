use {rl_ast::nodes::ExpressionKind, rl_ast::statements::StatementKind, rl_utils::span::Span};

use crate::common;

#[test]
fn match_literal_and_wildcard() {
    // "match x { 1 => {0} _ => {1} }"
    let (ast, statements) = common::parse("match x { 1 => {0} _ => {1} }");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Match { value, arms } => {
            common::assert_expr(
                &ast,
                *value,
                ExpressionKind::Identifier("x".to_string()),
                Span::new(6, 7),
            );
            assert_eq!(arms.len(), 2, "expected exactly two arms");
            common::assert_match_arm(
                &arms[0],
                &ast,
                Some((ExpressionKind::Integer(1), Span::new(10, 11))),
                (
                    ExpressionKind::Integer(0),
                    Span::new(16, 17),
                    Span::new(16, 17),
                ),
            );
            common::assert_match_arm(
                &arms[1],
                &ast,
                None,
                (
                    ExpressionKind::Integer(1),
                    Span::new(25, 26),
                    Span::new(25, 26),
                ),
            );
        }
        other => panic!("expected Match, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 29));
}

#[test]
fn match_multiple_literals() {
    // "match x { 1 => {0} 2 => {1} }"
    let (ast, statements) = common::parse("match x { 1 => {0} 2 => {1} }");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Match { value, arms } => {
            common::assert_expr(
                &ast,
                *value,
                ExpressionKind::Identifier("x".to_string()),
                Span::new(6, 7),
            );
            assert_eq!(arms.len(), 2, "expected exactly two arms");
            common::assert_match_arm(
                &arms[0],
                &ast,
                Some((ExpressionKind::Integer(1), Span::new(10, 11))),
                (
                    ExpressionKind::Integer(0),
                    Span::new(16, 17),
                    Span::new(16, 17),
                ),
            );
            common::assert_match_arm(
                &arms[1],
                &ast,
                Some((ExpressionKind::Integer(2), Span::new(19, 20))),
                (
                    ExpressionKind::Integer(1),
                    Span::new(25, 26),
                    Span::new(25, 26),
                ),
            );
        }
        other => panic!("expected Match, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 29));
}
