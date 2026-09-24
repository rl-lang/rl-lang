use rl_ast::{
    nodes::ExpressionKind,
    statements::{FunctionAttribute, Param, StatementKind, TypeAnnotation},
};

use crate::common::{self, span_of, span_of_last, span_whole};

#[test]
fn fn_simple() {
    let source = "fn x (int x) {return x}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name,
            params,
            return_type,
            attribute,
            body,
            item_attributes: _,
            requires: _,
            ensures: _,
        } => {
            assert_eq!(name, "x");
            assert_eq!(
                params,
                &vec![Param {
                    param_name: "x".to_string(),
                    param_type: TypeAnnotation::Int,
                    refinement: None,
                }]
            );
            assert_eq!(*return_type, TypeAnnotation::Null);
            assert_eq!(*attribute, None);
            assert_eq!(body.len(), 1, "expected exactly one body statement");
            common::assert_return(
                &body[0],
                &ast,
                Some((
                    ExpressionKind::Identifier("x".to_string()),
                    span_of_last(source, "x"),
                )),
                span_of(source, "return x"),
            );
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn fn_fn_param() {
    let source = "fn x (fn x, int y) {return x(y)}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name,
            params,
            return_type,
            attribute,
            body,
            item_attributes: _,
            requires: _,
            ensures: _,
        } => {
            assert_eq!(name, "x");
            assert_eq!(
                params,
                &vec![
                    Param {
                        param_name: "x".to_string(),
                        param_type: TypeAnnotation::Fn,
                        refinement: None,
                    },
                    Param {
                        param_name: "y".to_string(),
                        param_type: TypeAnnotation::Int,
                        refinement: None,
                    },
                ]
            );
            assert_eq!(*return_type, TypeAnnotation::Null);
            assert_eq!(*attribute, None);
            assert_eq!(body.len(), 1, "expected exactly one body statement");

            assert_eq!(body[0].span, span_of(source, "return x(y)"));
            match &body[0].kind {
                StatementKind::Return(Some(id)) => {
                    let expr = ast.exprs.get(*id);
                    assert_eq!(expr.span, span_of(source, "x(y)"));
                    match &expr.kind {
                        ExpressionKind::Call { path, args } => {
                            assert_eq!(path, &vec!["x".to_string()]);
                            assert_eq!(args.len(), 1);
                            common::assert_expr(
                                &ast,
                                args[0],
                                ExpressionKind::Identifier("y".to_string()),
                                span_of_last(source, "y"),
                            );
                        }
                        other => panic!("expected Call, got {:?}", other),
                    }
                }
                other => panic!("expected Return(Some(_)), got {:?}", other),
            }
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn dec_fn_lambda() {
    let source = "dec fn x = fn(int x) {return x}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::VariableDeclaration {
            name,
            type_annotation,
            value,
            ..
        } => {
            assert_eq!(name, "x");
            assert_eq!(*type_annotation, TypeAnnotation::Fn);

            let expr = ast.exprs.get(*value);
            assert_eq!(expr.span, span_of(source, "fn(int x) {return x}"));
            match &expr.kind {
                ExpressionKind::Lambda {
                    params,
                    return_type,
                    body,
                } => {
                    assert_eq!(
                        params,
                        &vec![Param {
                            param_name: "x".to_string(),
                            param_type: TypeAnnotation::Int,
                            refinement: None,
                        }]
                    );
                    assert_eq!(*return_type, None);
                    assert_eq!(body.len(), 1, "expected exactly one body statement");
                    common::assert_return(
                        &body[0],
                        &ast,
                        Some((
                            ExpressionKind::Identifier("x".to_string()),
                            span_of_last(source, "x"),
                        )),
                        span_of(source, "return x"),
                    );
                }
                other => panic!("expected Lambda, got {:?}", other),
            }
        }
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn entry_attribute_marks_function() {
    let (_ast, statements) = common::parse("!#[entry]\nfn start () {return 1}");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name, attribute, ..
        } => {
            assert_eq!(name, "start");
            assert_eq!(*attribute, Some(FunctionAttribute::Entry));
        }
        other => panic!("expected function declaration, got {:?}", other),
    }
}

#[test]
fn init_attribute_without_priority() {
    let (_ast, statements) = common::parse("!#[init]\nfn setup () {return 1}");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name, attribute, ..
        } => {
            assert_eq!(name, "setup");
            assert_eq!(*attribute, Some(FunctionAttribute::Init(None)));
        }
        other => panic!("expected function declaration, got {:?}", other),
    }
}

#[test]
fn init_attribute_with_priority() {
    let (_ast, statements) = common::parse("!#[init=2]\nfn setup () {return 1}");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name, attribute, ..
        } => {
            assert_eq!(name, "setup");
            assert_eq!(*attribute, Some(FunctionAttribute::Init(Some(2))));
        }
        other => panic!("expected function declaration, got {:?}", other),
    }
}

#[test]
fn final_attribute_with_priority() {
    let (_ast, statements) = common::parse("!#[final=0]\nfn teardown () {return 1}");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name, attribute, ..
        } => {
            assert_eq!(name, "teardown");
            assert_eq!(*attribute, Some(FunctionAttribute::Final(Some(0))));
        }
        other => panic!("expected function declaration, got {:?}", other),
    }
}

#[test]
fn init_attribute_priority_must_be_a_number() {
    let err = common::parse_assert_err("!#[init=hi]\nfn setup () {return 1}");
    assert!(
        err.contains("expected a number after `=`"),
        "unexpected error message: {err}"
    );
}

#[test]
fn entry_attribute_rejects_priority() {
    let err = common::parse_assert_err("!#[entry=1]\nfn start () {return 1}");
    assert!(
        err.contains("`!#[entry]` does not take a priority"),
        "unexpected error message: {err}"
    );
}

#[test]
fn test_attribute_rejects_priority() {
    let err = common::parse_assert_err("!#[test=1]\nfn check () {return 1}");
    assert!(
        err.contains("`!#[test]` does not take a priority"),
        "unexpected error message: {err}"
    );
}

#[test]
fn fn_handle_return_annotation() {
    let (_, statements) = common::parse("fn grab (string p) -> result[handle] { open(p) }");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { return_type, .. } => {
            assert_eq!(
                *return_type,
                TypeAnnotation::Result(Box::new(TypeAnnotation::HandleInfer))
            );
        }
        other => panic!("expected fn decl, got {other:?}"),
    }
}

#[test]
fn fn_handle_param() {
    let (_, statements) = common::parse("fn shut (handle h) { close(h) }");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { params, .. } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].param_type, TypeAnnotation::HandleInfer);
        }
        other => panic!("expected fn decl, got {other:?}"),
    }
}
