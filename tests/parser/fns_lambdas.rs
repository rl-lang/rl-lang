use rl_lang::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Param, Statement, StatementKind, TypeAnnotation},
    },
    utils::span::Span,
};

use crate::common;

#[test]
fn fn_simple() {
    let statements = common::parse("fn x (int x) {return x}");
    let expected = Statement::new(
        StatementKind::FunctionDeclaration {
            name: "x".to_string(),
            params: vec![Param {
                param_name: "x".to_string(),
                param_type: TypeAnnotation::Int,
            }],
            return_type: TypeAnnotation::Null,
            body: vec![Statement::new(
                StatementKind::Return(Some(Expression::new(
                    ExpressionKind::Identifier("x".to_string()),
                    Span::new(21, 22),
                ))),
                Span::new(14, 22),
            )],
        },
        Span::new(0, 23),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn fn_fn_param() {
    let statements = common::parse("fn x (fn x, int y) {return x(y)}");
    let expected = Statement::new(
        StatementKind::FunctionDeclaration {
            name: "x".to_string(),
            params: vec![
                Param {
                    param_name: "x".to_string(),
                    param_type: TypeAnnotation::Fn,
                },
                Param {
                    param_name: "y".to_string(),
                    param_type: TypeAnnotation::Int,
                },
            ],
            return_type: TypeAnnotation::Null,
            body: vec![Statement::new(
                StatementKind::Return(Some(Expression::new(
                    ExpressionKind::Call {
                        path: vec!["x".to_string()],
                        args: vec![Expression::new(
                            ExpressionKind::Identifier("y".to_string()),
                            Span::new(29, 30),
                        )],
                    },
                    Span::new(27, 31),
                ))),
                Span::new(20, 31),
            )],
        },
        Span::new(0, 32),
    );
    assert_eq!(statements, vec![expected]);
}

#[test]
fn dec_fn_lambda() {
    let statements = common::parse("dec fn x = fn(int x) {return x}");
    let expected = Statement::new(
        StatementKind::VariableDeclaration {
            name: "x".to_string(),
            type_annotation: TypeAnnotation::Fn,
            value: Expression::new(
                ExpressionKind::Lambda {
                    params: vec![Param {
                        param_name: "x".to_string(),
                        param_type: TypeAnnotation::Int,
                    }],
                    return_type: None,
                    body: vec![Statement::new(
                        StatementKind::Return(Some(Expression::new(
                            ExpressionKind::Identifier("x".to_string()),
                            Span::new(29, 30),
                        ))),
                        Span::new(22, 30),
                    )],
                },
                Span::new(11, 31),
            ),
        },
        Span::new(0, 31),
    );
    assert_eq!(statements, vec![expected]);
}
