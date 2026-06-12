use rl_lang::{
    ast::statements::{Statement, StatementKind},
    utils::span::Span,
};

use crate::common;

#[test]
fn import_simple() {
    let statements = common::parse("get x from y");
    let expected = Statement::new(
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string()],
            names: vec!["x".to_string()],
        },
        Span::new(0, 12),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn import_path() {
    let statements = common::parse("get x from y::z");
    let expected = Statement::new(
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string(), "z".to_string()],
            names: vec!["x".to_string()],
        },
        Span::new(0, 15),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn import_multi() {
    let statements = common::parse("get x, z from y");
    let expected = Statement::new(
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string()],
            names: vec!["x".to_string(), "z".to_string()],
        },
        Span::new(0, 15),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn import_multi_path() {
    let statements = common::parse("get x, z from y::w");
    let expected = Statement::new(
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string(), "w".to_string()],
            names: vec!["x".to_string(), "z".to_string()],
        },
        Span::new(0, 18),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn import_file() {
    // get x  (no `from`, single segment — treat as file import)
    let statements = common::parse("get x");
    let expected = Statement::new(
        StatementKind::ImportFile {
            path: vec!["x".to_string()],
        },
        Span::new(0, 5),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn import_file_path() {
    // get x::y
    let statements = common::parse("get x::y");
    let expected = Statement::new(
        StatementKind::ImportFile {
            path: vec!["x".to_string(), "y".to_string()],
        },
        Span::new(0, 8),
    );
    assert_eq!(statements, vec![expected]);
}
