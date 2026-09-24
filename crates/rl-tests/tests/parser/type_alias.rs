use rl_ast::statements::{StatementKind, TypeAnnotation};

use crate::common::{parse, parse_assert_err};

#[test]
fn type_alias_registers_and_substitutes() {
    let (ast, statements) = parse("type number int\ndec number x = 1\n");
    assert_eq!(
        ast.type_aliases.get("number"),
        Some(&TypeAnnotation::Int),
    );
    assert_eq!(statements.len(), 2);
    match &statements[1].kind {
        StatementKind::VariableDeclaration {
            type_annotation, ..
        } => {
            assert_eq!(*type_annotation, TypeAnnotation::Int);
        }
        other => panic!("expected dec, got {other:?}"),
    }
}

#[test]
fn type_alias_tuple_target() {
    let (ast, _) = parse("type point (int, int, int)\n");
    assert_eq!(
        ast.type_aliases.get("point"),
        Some(&TypeAnnotation::Tuple(std::rc::Rc::new(vec![
            TypeAnnotation::Int,
            TypeAnnotation::Int,
            TypeAnnotation::Int,
        ]))),
    );
}

#[test]
fn type_alias_in_cast() {
    let (_, statements) = parse("type number int\ndec x = 1 as number\n");
    assert_eq!(statements.len(), 2);
}

#[test]
fn duplicate_type_alias_errors() {
    let msg = parse_assert_err("type a int\ntype a int\n");
    assert!(msg.contains("already defined"), "{msg}");
}

#[test]
fn use_before_type_alias_errors() {
    let msg = parse_assert_err("dec point p = (1, 2)\ntype point (int, int)\n");
    assert!(!msg.is_empty(), "expected an error");
}

#[test]
fn deprecated_alias_warns_on_use() {
    let checker = crate::common::check(
        "!#[deprecated(\"use num\")]\ntype oldnum int\ndec oldnum n = 5\n",
    );
    assert!(checker.errors.is_empty(), "{:?}", checker.errors);
    let warnings: Vec<String> = checker
        .warnings
        .iter()
        .map(|e| e.message().to_string())
        .collect();
    assert!(
        warnings
            .iter()
            .any(|m| m.contains("oldnum") && m.contains("use num")),
        "{warnings:?}",
    );
}
