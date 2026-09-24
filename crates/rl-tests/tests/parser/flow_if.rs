use rl_ast::{nodes::ExpressionKind, statements::StatementKind};

use crate::common::{self, span_of, span_of_last, span_whole};

#[test]
fn if_simple() {
    let source = "if (true) {0}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((
                    ExpressionKind::Bool(true),
                    span_of(source, "true"),
                    span_of(source, "(true)"),
                )),
                (
                    ExpressionKind::Integer(0),
                    span_of(source, "0"),
                    span_of(source, "0"),
                ),
                span_whole(source),
            );
            assert!(else_branch.is_none());
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn if_else() {
    let source = "if (true) {1} else {0}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((
                    ExpressionKind::Bool(true),
                    span_of(source, "true"),
                    span_of(source, "(true)"),
                )),
                (
                    ExpressionKind::Integer(1),
                    span_of(source, "1"),
                    span_of(source, "1"),
                ),
                span_of(source, "if (true) {1}"),
            );
            let else_branch = else_branch.as_ref().expect("expected else branch");
            common::assert_branch(
                else_branch,
                &ast,
                None,
                (
                    ExpressionKind::Integer(0),
                    span_of(source, "0"),
                    span_of(source, "0"),
                ),
                span_of(source, "else {0}"),
            );
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn else_brace_on_next_line() {
    // Allman layout: newline between `else` and `{` parses
    let source = "if x\n{\ny = 1\n}\nelse\n{\ny = 2\n}";
    let (_, statements) = common::parse(source);
    match &statements[0].kind {
        StatementKind::Conditional { .. } => {}
        other => panic!("expected Conditional, got {:?}", other),
    }
}

// else if is now a nested Conditional inside else_branch
#[test]
fn if_else_if() {
    let source = "if (true) {1} else if (false) {2}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((
                    ExpressionKind::Bool(true),
                    span_of(source, "true"),
                    span_of(source, "(true)"),
                )),
                (
                    ExpressionKind::Integer(1),
                    span_of(source, "1"),
                    span_of(source, "1"),
                ),
                span_of(source, "if (true) {1}"),
            );
            let else_branch = else_branch.as_ref().expect("expected else branch");
            assert_eq!(else_branch.span, span_of(source, "if (false) {2}"));
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
                            span_of(source, "false"),
                            span_of(source, "(false)"),
                        )),
                        (
                            ExpressionKind::Integer(2),
                            span_of(source, "2"),
                            span_of(source, "2"),
                        ),
                        span_of(source, "if (false) {2}"),
                    );
                    assert!(else_branch.is_none());
                }
                other => panic!("expected nested Conditional, got {:?}", other),
            }
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn if_else_if_else() {
    let source = "if (true) {1} else if (false) {2} else {0}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            common::assert_branch(
                if_branch,
                &ast,
                Some((
                    ExpressionKind::Bool(true),
                    span_of(source, "true"),
                    span_of(source, "(true)"),
                )),
                (
                    ExpressionKind::Integer(1),
                    span_of(source, "1"),
                    span_of(source, "1"),
                ),
                span_of(source, "if (true) {1}"),
            );
            let else_branch = else_branch.as_ref().expect("expected else branch");
            assert_eq!(else_branch.span, span_of(source, "if (false) {2} else {0}"));
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
                            span_of(source, "false"),
                            span_of(source, "(false)"),
                        )),
                        (
                            ExpressionKind::Integer(2),
                            span_of(source, "2"),
                            span_of(source, "2"),
                        ),
                        span_of(source, "if (false) {2}"),
                    );
                    let else_branch = else_branch.as_ref().expect("expected inner else branch");
                    common::assert_branch(
                        else_branch,
                        &ast,
                        None,
                        (
                            ExpressionKind::Integer(0),
                            span_of_last(source, "0"),
                            span_of_last(source, "0"),
                        ),
                        span_of(source, "else {0}"),
                    );
                }
                other => panic!("expected nested Conditional, got {:?}", other),
            }
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

// "if (true) { if (false) {0} else {1} } else {0}"
#[test]
fn if_nested() {
    let source = "if (true) { if (false) {0} else {1} } else {0}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            assert_eq!(
                if_branch.span,
                span_of(source, "if (true) { if (false) {0} else {1} }")
            );
            match &if_branch.kind {
                StatementKind::ConditionalBranch {
                    condition, body, ..
                } => {
                    let condition = condition.expect("expected condition");
                    common::assert_grouping(
                        &ast,
                        condition,
                        ExpressionKind::Bool(true),
                        span_of(source, "true"),
                        span_of(source, "(true)"),
                    );
                    assert_eq!(body.len(), 1, "expected exactly one body statement");
                    assert_eq!(body[0].span, span_of(source, "if (false) {0} else {1}"));
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
                                    span_of(source, "false"),
                                    span_of(source, "(false)"),
                                )),
                                (
                                    ExpressionKind::Integer(0),
                                    span_of(source, "0"),
                                    span_of(source, "0"),
                                ),
                                span_of(source, "if (false) {0}"),
                            );
                            let else_branch =
                                else_branch.as_ref().expect("expected inner else branch");
                            common::assert_branch(
                                else_branch,
                                &ast,
                                None,
                                (
                                    ExpressionKind::Integer(1),
                                    span_of(source, "1"),
                                    span_of(source, "1"),
                                ),
                                span_of(source, "else {1}"),
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
                    span_of_last(source, "0"),
                    span_of_last(source, "0"),
                ),
                span_of_last(source, "else {0}"),
            );
        }
        other => panic!("expected Conditional, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}
