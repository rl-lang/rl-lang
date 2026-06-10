use rl_lang::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind},
    },
    utils::span::Span,
};

use crate::common;

#[test]
fn if_simple() {
    let statements = common::parse("if (true) {0}");
    let expected = Statement::new(
        StatementKind::Conditional {
            if_branch: Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: Some(Expression::new(
                        ExpressionKind::Grouping(Box::new(Expression::new(
                            ExpressionKind::Bool(true),
                            Span::new(4, 8),
                        ))),
                        Span::new(3, 9),
                    )),
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(0),
                            Span::new(11, 12),
                        )),
                        Span::new(11, 12),
                    )],
                },
                Span::new(0, 13),
            )),
            elseif_branch: Some(vec![]),
            else_branch: None,
        },
        Span::new(0, 13),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn if_else() {
    let statements = common::parse("if (true) {1} else {0}");
    let expected = Statement::new(
        StatementKind::Conditional {
            if_branch: Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: Some(Expression::new(
                        ExpressionKind::Grouping(Box::new(Expression::new(
                            ExpressionKind::Bool(true),
                            Span::new(4, 8),
                        ))),
                        Span::new(3, 9),
                    )),
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(1),
                            Span::new(11, 12),
                        )),
                        Span::new(11, 12),
                    )],
                },
                Span::new(0, 13),
            )),
            elseif_branch: Some(vec![]),
            else_branch: Some(Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: None,
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(0),
                            Span::new(20, 21),
                        )),
                        Span::new(20, 21),
                    )],
                },
                Span::new(14, 22),
            ))),
        },
        Span::new(0, 22),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn if_else_if() {
    let statements = common::parse("if (true) {1} else if (false) {2}");
    let expected = Statement::new(
        StatementKind::Conditional {
            if_branch: Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: Some(Expression::new(
                        ExpressionKind::Grouping(Box::new(Expression::new(
                            ExpressionKind::Bool(true),
                            Span::new(4, 8),
                        ))),
                        Span::new(3, 9),
                    )),
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(1),
                            Span::new(11, 12),
                        )),
                        Span::new(11, 12),
                    )],
                },
                Span::new(0, 13),
            )),
            elseif_branch: Some(vec![Statement::new(
                StatementKind::ConditionalBranch {
                    condition: Some(Expression::new(
                        ExpressionKind::Grouping(Box::new(Expression::new(
                            ExpressionKind::Bool(false),
                            Span::new(23, 28),
                        ))),
                        Span::new(22, 29),
                    )),
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(2),
                            Span::new(31, 32),
                        )),
                        Span::new(31, 32),
                    )],
                },
                Span::new(14, 33),
            )]),
            else_branch: None,
        },
        Span::new(0, 33),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn if_else_if_else() {
    let statements = common::parse("if (true) {1} else if (false) {2} else {0}");
    let expected = Statement::new(
        StatementKind::Conditional {
            if_branch: Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: Some(Expression::new(
                        ExpressionKind::Grouping(Box::new(Expression::new(
                            ExpressionKind::Bool(true),
                            Span::new(4, 8),
                        ))),
                        Span::new(3, 9),
                    )),
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(1),
                            Span::new(11, 12),
                        )),
                        Span::new(11, 12),
                    )],
                },
                Span::new(0, 13),
            )),
            elseif_branch: Some(vec![Statement::new(
                StatementKind::ConditionalBranch {
                    condition: Some(Expression::new(
                        ExpressionKind::Grouping(Box::new(Expression::new(
                            ExpressionKind::Bool(false),
                            Span::new(23, 28),
                        ))),
                        Span::new(22, 29),
                    )),
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(2),
                            Span::new(31, 32),
                        )),
                        Span::new(31, 32),
                    )],
                },
                Span::new(14, 33),
            )]),
            else_branch: Some(Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: None,
                    body: vec![Statement::new(
                        StatementKind::Expression(Expression::new(
                            ExpressionKind::Integer(0),
                            Span::new(40, 41),
                        )),
                        Span::new(40, 41),
                    )],
                },
                Span::new(34, 42),
            ))),
        },
        Span::new(0, 42),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn if_nested() {
    let statements = common::parse("if (true) { if (false) {0} else {1} } else {0}");
    let expected = Statement::new(
        StatementKind::Conditional {
            if_branch: Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: Some(Expression::new(
                        ExpressionKind::Grouping(Box::new(Expression::new(
                            ExpressionKind::Bool(true),
                            Span::new(4, 8),
                        ))),
                        Span::new(3, 9),
                    )),
                    body: vec![Statement::new(
                        StatementKind::Conditional {
                            if_branch: Box::new(Statement::new(
                                StatementKind::ConditionalBranch {
                                    condition: Some(Expression::new(
                                        ExpressionKind::Grouping(Box::new(Expression::new(
                                            ExpressionKind::Bool(false),
                                            Span::new(16, 21),
                                        ))),
                                        Span::new(15, 22),
                                    )),
                                    body: vec![Statement::new(
                                        StatementKind::Expression(Expression::new(
                                            ExpressionKind::Integer(0),
                                            Span::new(24, 25),
                                        )),
                                        Span::new(24, 25),
                                    )],
                                },
                                Span::new(12, 26),
                            )),
                            elseif_branch: Some(vec![]),
                            else_branch: Some(Box::new(Statement::new(
                                StatementKind::ConditionalBranch {
                                    condition: None,
                                    body: vec![Statement::new(
                                        StatementKind::Expression(Expression::new(
                                            ExpressionKind::Integer(0),
                                            Span::new(44, 45),
                                        )),
                                        Span::new(44, 45),
                                    )],
                                },
                                Span::new(38, 46),
                            ))),
                        },
                        Span::new(12, 46),
                    )],
                },
                Span::new(0, 46),
            )),
            elseif_branch: Some(vec![]),
            else_branch: None,
        },
        Span::new(0, 46),
    );
    assert_eq!(statements, vec![expected]);
}
