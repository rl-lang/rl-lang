use rl_lang::{
    ast::{nodes::ExpressionKind, statements::StatementKind, statements::TypeAnnotation},
    lexer::tokentypes::TokenType,
    utils::span::Span,
};

use crate::{assert_for_range, common};

#[test]
fn for_c() {
    let (ast, statements) = common::parse("for [int i = 1, i < 10, i += 1] {0}");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::For {
            initializer,
            condition,
            increment,
            body,
        } => {
            match &initializer.kind {
                StatementKind::VariableDeclaration {
                    name,
                    type_annotation,
                    value,
                } => {
                    assert_eq!(name, "i");
                    assert_eq!(*type_annotation, TypeAnnotation::Int);
                    common::assert_expr(
                        &ast,
                        *value,
                        ExpressionKind::Integer(1),
                        Span::new(13, 14),
                    );
                }
                other => panic!("expected VariableDeclaration, got {:?}", other),
            }
            assert_eq!(initializer.span, Span::new(5, 14));

            common::assert_binary(
                &ast,
                *condition,
                ExpressionKind::Identifier("i".to_string()),
                Span::new(16, 17),
                TokenType::Less,
                ExpressionKind::Integer(10),
                Span::new(20, 22),
                Span::new(16, 22),
            );

            common::assert_assign(&ast, *increment, "i", Span::new(24, 30), |ast, value_id| {
                common::assert_binary(
                    ast,
                    value_id,
                    ExpressionKind::Identifier("i".to_string()),
                    Span::new(24, 25),
                    TokenType::Plus,
                    ExpressionKind::Integer(1),
                    Span::new(29, 30),
                    Span::new(24, 30),
                );
            });

            assert_eq!(body.len(), 1, "expected exactly one body statement");
            common::assert_single_expr_stmt(
                &body[0],
                &ast,
                ExpressionKind::Integer(0),
                Span::new(33, 34),
                Span::new(33, 34),
            );
        }
        other => panic!("expected For, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 35));
}

#[test]
fn for_range() {
    assert_for_range!(
        "for i in 1..10 {0}",
        variable: "i",
        range: vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Span::new(0, 14),
        body_expr: ExpressionKind::Integer(0), Span::new(16, 17), Span::new(16, 17),
        span: Span::new(0, 18),
    );
}

#[test]
fn for_iterable() {
    assert_for_range!(
        "for i in [1,2,3,4,5,6,7,8,9] {0}",
        variable: "i",
        range: vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Span::new(0, 28),
        body_expr: ExpressionKind::Integer(0), Span::new(30, 31), Span::new(30, 31),
        span: Span::new(0, 32),
    );
}
