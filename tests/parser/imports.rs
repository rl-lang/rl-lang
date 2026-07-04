use rl_lang::{ast::statements::StatementKind, utils::span::Span};

use crate::{assert_stmt, common};

#[test]
fn import_simple() {
    assert_stmt!(
        "get x from y",
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string()],
            names: vec!["x".to_string()],
        },
        Span::new(0, 12),
    );
}

#[test]
fn import_path() {
    assert_stmt!(
        "get x from y::z",
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string(), "z".to_string()],
            names: vec!["x".to_string()],
        },
        Span::new(0, 15),
    );
}

#[test]
fn import_multi() {
    assert_stmt!(
        "get x, z from y",
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string()],
            names: vec!["x".to_string(), "z".to_string()],
        },
        Span::new(0, 15),
    );
}

#[test]
fn import_multi_path() {
    assert_stmt!(
        "get x, z from y::w",
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string(), "w".to_string()],
            names: vec!["x".to_string(), "z".to_string()],
        },
        Span::new(0, 18),
    );
}

#[test]
fn import_file() {
    // get x  (no `from`, single segment - treat as file import)
    assert_stmt!(
        "get x",
        StatementKind::ImportFile {
            path: vec!["x".to_string()],
        },
        Span::new(0, 5),
    );
}

#[test]
fn import_file_path() {
    // get x::y
    assert_stmt!(
        "get x::y",
        StatementKind::ImportFile {
            path: vec!["x".to_string(), "y".to_string()],
        },
        Span::new(0, 8),
    );
}
