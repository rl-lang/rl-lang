use crate::common;
use rl_ast::statements::{Param, StatementKind, TypeAnnotation};

#[test]
fn fn_multiline_params() {
    let source = "\
fn add(
    int x,
    int y
) -> int {
    return x + y
}";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name,
            params,
            return_type,
            ..
        } => {
            assert_eq!(name, "add");
            assert_eq!(
                params,
                &vec![
                    Param { param_name: "x".to_string(), param_type: TypeAnnotation::Int, refinement: None },
                    Param { param_name: "y".to_string(), param_type: TypeAnnotation::Int, refinement: None },
                ]
            );
            assert_eq!(*return_type, TypeAnnotation::Int);
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}

#[test]
fn fn_multiline_return_type() {
    let source = "\
fn foo() ->
    int {
    return 1
}";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { return_type, .. } => {
            assert_eq!(*return_type, TypeAnnotation::Int);
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}

#[test]
fn fn_multiline_body() {
    let source = "\
fn compute() -> int {
    dec int x = 10
    dec int y = 20
    return x + y
}";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { body, .. } => {
            assert_eq!(body.len(), 3);
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}

#[test]
fn lambda_multiline_params() {
    let source = "\
dec fn add = fn(
    int x,
    int y
) {
    return x + y
}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::VariableDeclaration { value, .. } => {
            match &ast.exprs.get(*value).kind {
                rl_ast::nodes::ExpressionKind::Lambda { params, body, .. } => {
                    assert_eq!(
                        params,
                        &vec![
                            Param { param_name: "x".to_string(), param_type: TypeAnnotation::Int, refinement: None },
                            Param { param_name: "y".to_string(), param_type: TypeAnnotation::Int, refinement: None },
                        ]
                    );
                    assert_eq!(body.len(), 1);
                }
                other => panic!("expected Lambda, got {:?}", other),
            }
        }
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
}

#[test]
fn lambda_multiline_with_arrow() {
    let source = "\
dec fn f = fn(
    int x
) ->
    int {
    return x
}";
    let (ast, statements) = common::parse(source);
    match &statements[0].kind {
        StatementKind::VariableDeclaration { value, .. } => {
            match &ast.exprs.get(*value).kind {
                rl_ast::nodes::ExpressionKind::Lambda { return_type, .. } => {
                    assert_eq!(*return_type, Some(TypeAnnotation::Int));
                }
                other => panic!("expected Lambda, got {:?}", other),
            }
        }
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
}

#[test]
fn fn_name_on_next_line() {
    let source = "\
fn
    myFunc() {
    return 1
}";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { name, .. } => {
            assert_eq!(name, "myFunc");
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}

#[test]
fn for_range_with_newline_before_dotdot() {
    let source = "\
for i in 0
..5 {
    return i
}";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::ForRange { variable, .. } => {
            assert_eq!(variable, "i");
        }
        other => panic!("expected ForRange, got {:?}", other),
    }
}

#[test]
fn get_multiline_import() {
    let source = "get\nmymodule";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::ImportFile { path } => {
            assert_eq!(path, &vec!["mymodule".to_string()]);
        }
        other => panic!("expected ImportFile, got {:?}", other),
    }
}

#[test]
fn array_literal_multiline() {
    let source = "\
dec int x = [
    1,
    2,
    3
]";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::VariableDeclaration { value, .. } => {
            match &ast.exprs.get(*value).kind {
                rl_ast::nodes::ExpressionKind::ArrayLiteral(items) => {
                    assert_eq!(items.len(), 3);
                }
                other => panic!("expected ArrayLiteral, got {:?}", other),
            }
        }
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
}

#[test]
fn map_literal_multiline() {
    let source = "\
dec map[string,int] x = {
    \"a\": 1,
    \"b\": 2
}";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::Map { entries, .. } => {
            assert_eq!(entries.len(), 2);
        }
        other => panic!("expected Map, got {:?}", other),
    }
}

#[test]
fn fn_multiline_no_return_type() {
    let source = "\
fn greet(
    string name
) {
    return name
}";
    let (_, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name,
            params,
            return_type,
            ..
        } => {
            assert_eq!(name, "greet");
            assert_eq!(params.len(), 1);
            assert_eq!(*return_type, TypeAnnotation::Null);
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}

#[test]
fn lambda_multiline_no_return_type() {
    let source = "\
dec fn greet = fn(
    string name
) {
    return name
}";
    let (ast, statements) = common::parse(source);
    match &statements[0].kind {
        StatementKind::VariableDeclaration { value, .. } => {
            match &ast.exprs.get(*value).kind {
                rl_ast::nodes::ExpressionKind::Lambda {
                    params,
                    return_type,
                    ..
                } => {
                    assert_eq!(params.len(), 1);
                    assert_eq!(*return_type, None);
                }
                other => panic!("expected Lambda, got {:?}", other),
            }
        }
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
}
