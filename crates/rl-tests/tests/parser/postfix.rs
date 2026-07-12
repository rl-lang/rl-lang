use {
    rl_ast::{
        nodes::ExpressionKind,
        statements::{StatementKind, TypeAnnotation},
    },
    rl_utils::span::Span,
};

use crate::common;

#[test]
fn index_simple() {
    let (ast, statements) = common::parse("arx[0]");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, Span::new(0, 6));
    match &expr.kind {
        ExpressionKind::Index { target, index } => {
            common::assert_expr(
                &ast,
                *target,
                ExpressionKind::Identifier("arx".to_string()),
                Span::new(0, 3),
            );
            common::assert_expr(&ast, *index, ExpressionKind::Integer(0), Span::new(4, 5));
        }
        other => panic!("expected Index, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 6));
}

#[test]
fn index_chained() {
    let (ast, statements) = common::parse("arx[0][1]");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, Span::new(0, 9));
    match &expr.kind {
        ExpressionKind::Index { target, index } => {
            common::assert_expr(&ast, *index, ExpressionKind::Integer(1), Span::new(7, 8));
            let inner = ast.exprs.get(*target);
            assert_eq!(inner.span, Span::new(0, 6));
            match &inner.kind {
                ExpressionKind::Index { target, index } => {
                    common::assert_expr(
                        &ast,
                        *target,
                        ExpressionKind::Identifier("arx".to_string()),
                        Span::new(0, 3),
                    );
                    common::assert_expr(&ast, *index, ExpressionKind::Integer(0), Span::new(4, 5));
                }
                other => panic!("expected inner Index, got {:?}", other),
            }
        }
        other => panic!("expected Index, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 9));
}

#[test]
fn index_assign() {
    let (ast, statements) = common::parse("arx[0] = 1");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, Span::new(0, 10));
    match &expr.kind {
        ExpressionKind::IndexAssign {
            target,
            index,
            value,
        } => {
            common::assert_expr(
                &ast,
                *target,
                ExpressionKind::Identifier("arx".to_string()),
                Span::new(0, 3),
            );
            common::assert_expr(&ast, *index, ExpressionKind::Integer(0), Span::new(4, 5));
            common::assert_expr(&ast, *value, ExpressionKind::Integer(1), Span::new(9, 10));
        }
        other => panic!("expected IndexAssign, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 10));
}

#[test]
fn method_call_simple() {
    let (ast, statements) = common::parse("x.foo(1)");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, Span::new(0, 8));
    match &expr.kind {
        ExpressionKind::MethodCall {
            caller,
            method,
            args,
        } => {
            common::assert_expr(
                &ast,
                *caller,
                ExpressionKind::Identifier("x".to_string()),
                Span::new(0, 1),
            );
            assert_eq!(method, &vec!["foo".to_string()]);
            assert_eq!(args.len(), 1);
            common::assert_expr(&ast, args[0], ExpressionKind::Integer(1), Span::new(6, 7));
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 8));
}

#[test]
fn cast_postfix() {
    let (ast, statements) = common::parse("x as int");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, Span::new(0, 8));
    match &expr.kind {
        ExpressionKind::Cast { value, target_type } => {
            assert_eq!(*target_type, TypeAnnotation::Int);
            common::assert_expr(
                &ast,
                *value,
                ExpressionKind::Identifier("x".to_string()),
                Span::new(0, 1),
            );
        }
        other => panic!("expected Cast, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 8));
}

#[test]
fn propagate_operator() {
    let (ast, statements) = common::parse("x?");
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, Span::new(0, 2));
    match &expr.kind {
        ExpressionKind::Propagate(inner) => {
            common::assert_expr(
                &ast,
                *inner,
                ExpressionKind::Identifier("x".to_string()),
                Span::new(0, 1),
            );
        }
        other => panic!("expected Propagate, got {:?}", other),
    }
    assert_eq!(statements[0].span, Span::new(0, 2));
}
