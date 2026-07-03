use rl_lang::{
    ast::{
        nodes::ExpressionKind,
        statements::{StatementKind, TypeAnnotation},
    },
    utils::span::Span,
};

use crate::{assert_array_decl, assert_decl, common};

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
    assert_decl!(
        "CONST int x = 1000",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CInt,
        value: ExpressionKind::Integer(1000), Span::new(14, 18),
        span: Span::new(0, 18),
    );
}

#[test]
fn dec_float() {
    assert_decl!(
        "dec float x = 1000.0",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Float,
        value: ExpressionKind::Float(1000.0), Span::new(14, 20),
        span: Span::new(0, 20),
    );
}

#[test]
fn const_float() {
    assert_decl!(
        "CONST float x = 1000.0",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CFloat,
        value: ExpressionKind::Float(1000.0), Span::new(16, 22),
        span: Span::new(0, 22),
    );
}

#[test]
fn dec_string() {
    assert_decl!(
        "dec string x = \"hi\"",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::String,
        value: ExpressionKind::String("hi".to_string()), Span::new(15, 19),
        span: Span::new(0, 19),
    );
}

#[test]
fn const_string() {
    assert_decl!(
        "CONST string x = \"hi\"",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CString,
        value: ExpressionKind::String("hi".to_string()), Span::new(17, 21),
        span: Span::new(0, 21),
    );
}

#[test]
fn dec_char() {
    assert_decl!(
        "dec char x = 'x'",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Char,
        value: ExpressionKind::Character('x'), Span::new(13, 16),
        span: Span::new(0, 16),
    );
}

#[test]
fn const_char() {
    assert_decl!(
        "CONST char x = 'x'",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CChar,
        value: ExpressionKind::Character('x'), Span::new(15, 18),
        span: Span::new(0, 18),
    );
}

#[test]
fn dec_bool() {
    assert_decl!(
        "dec bool x = true",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Bool,
        value: ExpressionKind::Bool(true), Span::new(13, 17),
        span: Span::new(0, 17),
    );
}

#[test]
fn const_bool() {
    assert_decl!(
        "CONST bool x = false",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CBool,
        value: ExpressionKind::Bool(false), Span::new(15, 20),
        span: Span::new(0, 20),
    );
}

#[test]
fn dec_byte() {
    assert_decl!(
        "dec byte x = 65 as byte",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Byte,
        value: ExpressionKind::Byte(65), Span::new(13, 15),
        span: Span::new(0, 15),
    );
}

#[test]
fn const_byte() {
    assert_decl!(
        "CONST byte x = 65 as byte",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CByte,
        value: ExpressionKind::Byte(65), Span::new(15, 17),
        span: Span::new(0, 17),
    );
}

#[test]
fn dec_array() {
    assert_array_decl!(
        "dec arr[int] x = [1]",
        StatementKind::Array,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        item: ExpressionKind::Integer(1), Span::new(18, 19),
        span: Span::new(0, 20),
    );
}

#[test]
fn const_array() {
    assert_array_decl!(
        "CONST arr[int] x = [1]",
        StatementKind::ConstantArray,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        item: ExpressionKind::Integer(1), Span::new(20, 21),
        span: Span::new(0, 22),
    );
}

// Empty lambda body/params means there's no nested ExprId to worry about,
// so this one *can* go through assert_decl! directly.
#[test]
fn dec_fn() {
    assert_decl!(
        "dec fn x = fn(){}",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Fn,
        value: ExpressionKind::Lambda { params: vec![], return_type: None, body: vec![] }, Span::new(11, 17),
        span: Span::new(0, 17),
    );
}

#[test]
fn const_fn() {
    assert_decl!(
        "CONST fn x = fn(){}",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Fn,
        value: ExpressionKind::Lambda { params: vec![], return_type: None, body: vec![] }, Span::new(13, 19),
        span: Span::new(0, 19),
    );
}
