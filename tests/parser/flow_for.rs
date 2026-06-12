use rl_lang::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind, TypeAnnotation},
    },
    lexer::tokentypes::TokenType,
    utils::span::Span,
};

use crate::common;

#[test]
fn for_c() {
    let statements = common::parse("for [int i = 1, i < 10, i += 1] {0}");
    let for_statement = Statement::new(
        StatementKind::For {
            initializer: Box::new(Statement::new(
                StatementKind::VariableDeclaration {
                    name: "i".to_string(),
                    type_annotation: TypeAnnotation::Int,
                    value: Expression::new(ExpressionKind::Integer(1), Span::new(13, 14)),
                },
                Span::new(5, 14),
            )),
            condition: Expression::new(
                ExpressionKind::Binary {
                    left: Box::new(Expression::new(
                        ExpressionKind::Identifier("i".to_string()),
                        Span::new(16, 17),
                    )),
                    operator: TokenType::Less,
                    right: Box::new(Expression::new(
                        ExpressionKind::Integer(10),
                        Span::new(20, 22),
                    )),
                },
                Span::new(16, 22),
            ),
            increment: Expression::new(
                ExpressionKind::Assign {
                    name: "i".to_string(),
                    value: Box::new(Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(Expression::new(
                                ExpressionKind::Identifier("i".to_string()),
                                Span::new(24, 25),
                            )),
                            operator: TokenType::Plus,
                            right: Box::new(Expression::new(
                                ExpressionKind::Integer(1),
                                Span::new(29, 30),
                            )),
                        },
                        Span::new(24, 30),
                    )),
                },
                Span::new(24, 30),
            ),
            body: vec![Statement::new(
                StatementKind::Expression(Expression::new(
                    ExpressionKind::Integer(0),
                    Span::new(33, 34),
                )),
                Span::new(33, 34),
            )],
        },
        Span::new(0, 35),
    );
    assert_eq!(statements, vec![for_statement]);
}

#[test]
fn for_range() {
    let statements = common::parse("for i in 1..10 {0}");
    let for_statement = Statement::new(
        StatementKind::ForRange {
            variable: "i".to_string(),
            range: Box::new(Statement::new(
                StatementKind::Range(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
                Span::new(0, 14),
            )),
            body: vec![Statement::new(
                StatementKind::Expression(Expression::new(
                    ExpressionKind::Integer(0),
                    Span::new(16, 17),
                )),
                Span::new(16, 17),
            )],
        },
        Span::new(0, 18),
    );
    assert_eq!(statements, vec![for_statement]);
}

#[test]
fn for_iterable() {
    let statements = common::parse("for i in [1,2,3,4,5,6,7,8,9] {0}");
    let for_statement = Statement::new(
        StatementKind::ForRange {
            variable: "i".to_string(),
            range: Box::new(Statement::new(
                StatementKind::Range(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
                Span::new(0, 28),
            )),
            body: vec![Statement::new(
                StatementKind::Expression(Expression::new(
                    ExpressionKind::Integer(0),
                    Span::new(30, 31),
                )),
                Span::new(30, 31),
            )],
        },
        Span::new(0, 32),
    );
    assert_eq!(statements, vec![for_statement]);
}
