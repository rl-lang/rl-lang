use rl_ast::statements::StatementKind;

use crate::assert_stmt;
use crate::common::{self, span_whole};

#[test]
fn import_simple() {
    let source = "get x from y";
    assert_stmt!(
        source,
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string()],
            names: vec!["x".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_path() {
    let source = "get x from y::z";
    assert_stmt!(
        source,
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string(), "z".to_string()],
            names: vec!["x".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_multi() {
    let source = "get x, z from y";
    assert_stmt!(
        source,
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string()],
            names: vec!["x".to_string(), "z".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_multi_path() {
    let source = "get x, z from y::w";
    assert_stmt!(
        source,
        StatementKind::ImportFileNamed {
            path: vec!["y".to_string(), "w".to_string()],
            names: vec!["x".to_string(), "z".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_file() {
    // get x  (no `from`, single segment - treat as file import)
    let source = "get x";
    assert_stmt!(
        source,
        StatementKind::ImportFile {
            path: vec!["x".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_file_path() {
    // get x::y
    let source = "get x::y";
    assert_stmt!(
        source,
        StatementKind::ImportFile {
            path: vec!["x".to_string(), "y".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_wildcard() {
    let source = "get * from std::math";
    assert_stmt!(
        source,
        StatementKind::Import {
            names: vec![],
            wildcard: true,
            path: vec!["std".to_string(), "math".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_aliased() {
    let source = "get sin as sine from std::math";
    assert_stmt!(
        source,
        StatementKind::Import {
            names: vec![("sin".to_string(), Some("sine".to_string()))],
            wildcard: false,
            path: vec!["std".to_string(), "math".to_string()],
        },
        span_whole(source),
    );
}

#[test]
fn import_mixed_alias_and_plain() {
    let source = "get sin as sine, cos from std::math";
    assert_stmt!(
        source,
        StatementKind::Import {
            names: vec![
                ("sin".to_string(), Some("sine".to_string())),
                ("cos".to_string(), None),
            ],
            wildcard: false,
            path: vec!["std".to_string(), "math".to_string()],
        },
        span_whole(source),
    );
}
