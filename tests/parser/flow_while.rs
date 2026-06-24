use rl_lang::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind},
    },
    utils::span::Span,
};

use crate::common;

#[test]
fn while_loop() {
    let statements = common::parse("while (true) {0}");
    let expected = Statement::new(
        StatementKind::While {
            condition: Expression::new(
                ExpressionKind::Grouping(Box::new(Expression::new(
                    ExpressionKind::Bool(true),
                    Span::new(7, 11),
                ))),
                Span::new(6, 12),
            ),
            body: vec![Statement::new(
                StatementKind::Expression(Expression::new(
                    ExpressionKind::Integer(0),
                    Span::new(14, 15),
                )),
                Span::new(14, 15),
            )],
        },
        Span::new(0, 16),
    );
    assert_eq!(statements, vec![expected]);
}
