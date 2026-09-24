use std::rc::Rc;

use rl_ast::statements::{StatementKind, TypeAnnotation};

use crate::common::{parse, parse_assert_err};

fn dec_annotation(source: &str) -> TypeAnnotation {
    let (_, statements) = parse(source);
    match &statements[0].kind {
        StatementKind::VariableDeclaration {
            type_annotation, ..
        } => type_annotation.clone(),
        other => panic!("expected dec, got {other:?}"),
    }
}

#[test]
fn any_registers_members() {
    assert_eq!(
        dec_annotation("dec any[int, string] x = 1\n"),
        TypeAnnotation::Any(Rc::new(vec![TypeAnnotation::Int, TypeAnnotation::String])),
    );
}

#[test]
fn any_dedupes_members() {
    assert_eq!(
        dec_annotation("dec any[int, string, int] x = 1\n"),
        TypeAnnotation::Any(Rc::new(vec![TypeAnnotation::Int, TypeAnnotation::String])),
    );
}

#[test]
fn any_flattens_nested() {
    assert_eq!(
        dec_annotation("dec any[int, any[string, bool]] x = 1\n"),
        TypeAnnotation::Any(Rc::new(vec![
            TypeAnnotation::Int,
            TypeAnnotation::String,
            TypeAnnotation::Bool,
        ])),
    );
}

#[test]
fn any_allows_trailing_comma() {
    assert_eq!(
        dec_annotation("dec any[int, string,] x = 1\n"),
        TypeAnnotation::Any(Rc::new(vec![TypeAnnotation::Int, TypeAnnotation::String])),
    );
}

#[test]
fn any_empty_errors() {
    let msg = parse_assert_err("dec any[] x = 1\n");
    assert!(msg.contains("at least one member"), "{msg}");
}

#[test]
fn any_single_member_errors() {
    let msg = parse_assert_err("dec any[int] x = 1\n");
    assert!(msg.contains("just that type"), "{msg}");
}

#[test]
fn any_bare_errors() {
    let msg = parse_assert_err("dec any x = 1\n");
    assert!(msg.contains("member list"), "{msg}");
}

#[test]
fn any_in_fn_signature() {
    let (_, statements) = parse("fn f(any[int, string] x) -> any[int, string] { x }\n");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            params,
            return_type,
            ..
        } => {
            assert_eq!(params.len(), 1);
            assert_eq!(
                params[0].param_type,
                TypeAnnotation::Any(Rc::new(vec![
                    TypeAnnotation::Int,
                    TypeAnnotation::String
                ])),
            );
            assert_eq!(
                *return_type,
                TypeAnnotation::Any(Rc::new(vec![
                    TypeAnnotation::Int,
                    TypeAnnotation::String
                ])),
            );
        }
        other => panic!("expected fn decl, got {other:?}"),
    }
}

#[test]
fn is_parses_with_type_target() {
    let (ast, statements) = parse("dec bool b = x is int\n");
    match &statements[0].kind {
        StatementKind::VariableDeclaration { value, .. } => {
            match &ast.exprs.get(*value).kind {
                rl_ast::nodes::ExpressionKind::Is { target_type, .. } => {
                    assert_eq!(*target_type, TypeAnnotation::Int);
                }
                other => panic!("expected Is, got {other:?}"),
            }
        }
        other => panic!("expected dec, got {other:?}"),
    }
}

#[test]
fn is_binds_like_equality() {
    // `a is int == true` groups as `(a is int) == true`
    let (ast, statements) = parse("dec bool b = a is int == true\n");
    match &statements[0].kind {
        StatementKind::VariableDeclaration { value, .. } => {
            match &ast.exprs.get(*value).kind {
                rl_ast::nodes::ExpressionKind::Binary { left, .. } => {
                    match &ast.exprs.get(*left).kind {
                        rl_ast::nodes::ExpressionKind::Is { .. } => {}
                        other => panic!("expected Is on the left, got {other:?}"),
                    }
                }
                other => panic!("expected Binary, got {other:?}"),
            }
        }
        other => panic!("expected dec, got {other:?}"),
    }
}

#[test]
fn is_union_target_rejected() {
    let msg = parse_assert_err("dec bool b = x is any[int, string]\n");
    assert!(msg.contains("one concrete type"), "{msg}");
}
