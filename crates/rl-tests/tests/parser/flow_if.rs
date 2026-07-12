use {
    rl_ast::{nodes::ExpressionKind, statements::StatementKind},
    rl_utils::span::Span,
};

use crate::common;

#[test]
fn if_simple() {
    let (ast, statements) = common::parse("if (true) {0}");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((ExpressionKind::Bool(true), Span::new(4, 8), Span::new(3, 9))),
                (
                    ExpressionKind::Integer(0),
                    Span::new(11, 12),
                    Span::new(11, 12),
                ),
                Span::new(0, 13),
            );
            assert!(else_branch.is_none());
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 13));
}

#[test]
fn if_else() {
    let (ast, statements) = common::parse("if (true) {1} else {0}");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((ExpressionKind::Bool(true), Span::new(4, 8), Span::new(3, 9))),
                (
                    ExpressionKind::Integer(1),
                    Span::new(11, 12),
                    Span::new(11, 12),
                ),
                Span::new(0, 13),
            );
            let else_branch = else_branch.as_ref().expect("expected else branch");
            common::assert_branch(
                else_branch,
                &ast,
                None,
                (
                    ExpressionKind::Integer(0),
                    Span::new(20, 21),
                    Span::new(20, 21),
                ),
                Span::new(14, 22),
            );
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 22));
}

// else if is now a nested Conditional inside else_branch
#[test]
fn if_else_if() {
    let (ast, statements) = common::parse("if (true) {1} else if (false) {2}");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((ExpressionKind::Bool(true), Span::new(4, 8), Span::new(3, 9))),
                (
                    ExpressionKind::Integer(1),
                    Span::new(11, 12),
                    Span::new(11, 12),
                ),
                Span::new(0, 13),
            );
            let else_branch = else_branch.as_ref().expect("expected else branch");
            assert_eq!(else_branch.span, Span::new(19, 33));
            match &else_branch.kind {
                StatementKind::Conditional {
                    if_branch,
                    else_branch,
                } => {
                    common::assert_branch(
                        if_branch,
                        &ast,
                        Some((
                            ExpressionKind::Bool(false),
                            Span::new(23, 28),
                            Span::new(22, 29),
                        )),
                        (
                            ExpressionKind::Integer(2),
                            Span::new(31, 32),
                            Span::new(31, 32),
                        ),
                        Span::new(19, 33),
                    );
                    assert!(else_branch.is_none());
                }
                other => panic!("expected nested Conditional, got {:?}", other),
            }
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 33));
}

#[test]
fn if_else_if_else() {
    let (ast, statements) = common::parse("if (true) {1} else if (false) {2} else {0}");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((ExpressionKind::Bool(true), Span::new(4, 8), Span::new(3, 9))),
                (
                    ExpressionKind::Integer(1),
                    Span::new(11, 12),
                    Span::new(11, 12),
                ),
                Span::new(0, 13),
            );
            let else_branch = else_branch.as_ref().expect("expected else branch");
            assert_eq!(else_branch.span, Span::new(19, 42));
            match &else_branch.kind {
                StatementKind::Conditional {
                    if_branch,
                    else_branch,
                } => {
                    common::assert_branch(
                        if_branch,
                        &ast,
                        Some((
                            ExpressionKind::Bool(false),
                            Span::new(23, 28),
                            Span::new(22, 29),
                        )),
                        (
                            ExpressionKind::Integer(2),
                            Span::new(31, 32),
                            Span::new(31, 32),
                        ),
                        Span::new(19, 33),
                    );
                    let else_branch = else_branch.as_ref().expect("expected inner else branch");
                    common::assert_branch(
                        else_branch,
                        &ast,
                        None,
                        (
                            ExpressionKind::Integer(0),
                            Span::new(40, 41),
                            Span::new(40, 41),
                        ),
                        Span::new(34, 42),
                    );
                }
                other => panic!("expected nested Conditional, got {:?}", other),
            }
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 42));
}

// "if (true) { if (false) {0} else {1} } else {0}"
#[test]
fn if_nested() {
    let (ast, statements) = common::parse("if (true) { if (false) {0} else {1} } else {0}");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            assert_eq!(if_branch.span, Span::new(0, 37));
            match &if_branch.kind {
                StatementKind::ConditionalBranch {
                    condition, body, ..
                } => {
                    let condition = condition.expect("expected condition");
                    common::assert_grouping(
                        &ast,
                        condition,
                        ExpressionKind::Bool(true),
                        Span::new(4, 8),
                        Span::new(3, 9),
                    );
                    assert_eq!(body.len(), 1, "expected exactly one body statement");
                    assert_eq!(body[0].span, Span::new(12, 35));
                    match &body[0].kind {
                        StatementKind::Conditional {
                            if_branch,
                            else_branch,
                        } => {
                            common::assert_branch(
                                if_branch,
                                &ast,
                                Some((
                                    ExpressionKind::Bool(false),
                                    Span::new(16, 21),
                                    Span::new(15, 22),
                                )),
                                (
                                    ExpressionKind::Integer(0),
                                    Span::new(24, 25),
                                    Span::new(24, 25),
                                ),
                                Span::new(12, 26),
                            );
                            let else_branch =
                                else_branch.as_ref().expect("expected inner else branch");
                            common::assert_branch(
                                else_branch,
                                &ast,
                                None,
                                (
                                    ExpressionKind::Integer(1),
                                    Span::new(33, 34),
                                    Span::new(33, 34),
                                ),
                                Span::new(27, 35),
                            );
                        }
                        other => panic!("expected inner Conditional, got {:?}", other),
                    }
                }
                other => panic!("expected ConditionalBranch, got {:?}", other),
            }
            let else_branch = else_branch.as_ref().expect("expected outer else branch");
            common::assert_branch(
                else_branch,
                &ast,
                None,
                (
                    ExpressionKind::Integer(0),
                    Span::new(44, 45),
                    Span::new(44, 45),
                ),
                Span::new(38, 46),
            );
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 46));
}
