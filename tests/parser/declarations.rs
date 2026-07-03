use rl_lang::{
    ast::{
        nodes::ExpressionKind,
        statements::{StatementKind, TypeAnnotation},
    },
    utils::span::Span,
};

use crate::{assert_decl, common};

#[test]
fn dec_int() {
    assert_decl!(
        "dec int x = 1000",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        value: ExpressionKind::Integer(1000), Span::new(12, 16),
        span: Span::new(0, 16),
    );
}

#[test]
fn const_int() {
    let (ast, statements) = common::parse("CONST int x = 1000");
    assert_eq!(statements.len(), 1);
    let StatementKind::ConstantDeclaration {
        name,
        type_annotation,
        value,
    } = &statements[0].kind
    else {
        panic!("expected ConstantDeclaration, got {:?}", statements[0].kind)
    };

    assert_eq!(name, "x");
    assert_eq!(*type_annotation, TypeAnnotation::CInt);
    assert_eq!(ast.exprs.get(*value).kind, ExpressionKind::Integer(1000));
    assert_eq!(ast.exprs.get(*value).span, Span::new(14, 18));
    assert_eq!(statements[0].span, Span::new(0, 18));
}

/*
#[test]
fn dec_float() {
    let statements = common::parse("dec float x = 0.0");
    let expected = Statement::new(
        StatementKind::VariableDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Float,
            value: Expression::new(ExpressionKind::Float(0.0), Span::new(14, 17)),
        },
        Span::new(0, 17),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn const_float() {
    let statements = common::parse("CONST float x = 0.0");
    let expected = Statement::new(
        StatementKind::ConstantDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::CFloat,
            value: Expression::new(ExpressionKind::Float(0.0), Span::new(16, 19)),
        },
        Span::new(0, 19),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn dec_string() {
    let statements = common::parse("dec string x = \"hi\"");
    let expected = Statement::new(
        StatementKind::VariableDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::String,
            value: Expression::new(ExpressionKind::String("hi".to_string()), Span::new(15, 19)),
        },
        Span::new(0, 19),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn const_string() {
    let statements = common::parse("CONST string x = \"hi\"");
    let expected = Statement::new(
        StatementKind::ConstantDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::CString,
            value: Expression::new(ExpressionKind::String("hi".to_string()), Span::new(17, 21)),
        },
        Span::new(0, 21),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn dec_byte() {
    let statements = common::parse("dec byte x = 65 as byte");
    let expected = Statement::new(
        StatementKind::VariableDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Byte,
            value: Expression::new(ExpressionKind::Byte(65), Span::new(13, 15)),
        },
        Span::new(0, 15),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn const_byte() {
    let statements = common::parse("CONST byte x = 65 as byte");
    let expected = Statement::new(
        StatementKind::ConstantDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::CByte,
            value: Expression::new(ExpressionKind::Byte(65), Span::new(15, 17)),
        },
        Span::new(0, 17),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn dec_char() {
    let statements = common::parse("dec char x = 'x'");
    let expected = Statement::new(
        StatementKind::VariableDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Char,
            value: Expression::new(ExpressionKind::Character('x'), Span::new(13, 16)),
        },
        Span::new(0, 16),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn const_char() {
    let statements = common::parse("CONST char x = 'x'");
    let expected = Statement::new(
        StatementKind::ConstantDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::CChar,
            value: Expression::new(ExpressionKind::Character('x'), Span::new(15, 18)),
        },
        Span::new(0, 18),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn dec_bool() {
    let statements = common::parse("dec bool x = true");
    let expected = Statement::new(
        StatementKind::VariableDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Bool,
            value: Expression::new(ExpressionKind::Bool(true), Span::new(13, 17)),
        },
        Span::new(0, 17),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn const_bool() {
    let statements = common::parse("CONST bool x = false");
    let expected = Statement::new(
        StatementKind::ConstantDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::CBool,
            value: Expression::new(ExpressionKind::Bool(false), Span::new(15, 20)),
        },
        Span::new(0, 20),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn dec_array() {
    let statements = common::parse("dec arr[int] x = [1]");
    let expected = Statement::new(
        StatementKind::Array {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Int,
            value: vec![Expression::new(
                ExpressionKind::Integer(1),
                Span::new(18, 19),
            )],
        },
        Span::new(0, 20),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn const_array() {
    let statements = common::parse("CONST arr[int] x = [1]");
    let expected = Statement::new(
        StatementKind::ConstantArray {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Int,
            value: vec![Expression::new(
                ExpressionKind::Integer(1),
                Span::new(20, 21),
            )],
        },
        Span::new(0, 22),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn dec_fn() {
    let statements = common::parse("dec fn x = fn(){}");
    let expected = Statement::new(
        StatementKind::VariableDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Fn,
            value: Expression::new(
                ExpressionKind::Lambda {
                    params: vec![],
                    return_type: None,
                    body: vec![],
                },
                Span::new(11, 17),
            ),
        },
        Span::new(0, 17),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn const_fn() {
    let statements = common::parse("CONST fn x = fn(){}");
    let expected = Statement::new(
        StatementKind::ConstantDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Fn,
            value: Expression::new(
                ExpressionKind::Lambda {
                    params: vec![],
                    return_type: None,
                    body: vec![],
                },
                Span::new(13, 19),
            ),
        },
        Span::new(0, 19),
    );
    assert_eq!(statements, vec![expected]);
}
*/
