use rl_ast::statements::{
    ContractClause, FunctionAttribute, Param, RefineOp, RefineOperand, StatementKind,
    TestParams, TypeAnnotation,
};

use crate::common;

fn parse_fn(source: &str) -> (rl_ast::Ast, Vec<rl_ast::statements::Statement>) {
    common::parse(source)
}

fn first_fn(source: &str) -> (rl_ast::Ast, String, Vec<Param>, TypeAnnotation, Vec<ContractClause>, Vec<ContractClause>) {
    let (ast, statements) = parse_fn(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name,
            params,
            return_type,
            requires,
            ensures,
            ..
        } => (
            ast,
            name.clone(),
            params.clone(),
            return_type.clone(),
            requires.clone(),
            ensures.clone(),
        ),
        other => panic!("expected FunctionDeclaration, got {other:?}"),
    }
}

#[test]
fn param_refinement_greater() {
    let (_, _, params, _, _, _) = first_fn("fn f(int amt: >0) -> int { return amt }");
    assert_eq!(params.len(), 1);
    let refinement = params[0].refinement.as_ref().expect("refinement");
    assert_eq!(refinement.op, RefineOp::Gt);
    assert_eq!(refinement.operand, RefineOperand::Integer(0));
}

#[test]
fn param_refinement_param_operand() {
    let (_, _, params, _, _, _) =
        first_fn("fn f(int amt, int balance: >=amt) -> int { return amt }");
    assert_eq!(params.len(), 2);
    assert!(params[0].refinement.is_none());
    let refinement = params[1].refinement.as_ref().expect("refinement");
    assert_eq!(refinement.op, RefineOp::Ge);
    assert_eq!(
        refinement.operand,
        RefineOperand::Param("amt".to_string())
    );
}

#[test]
fn param_refinement_string_and_bool() {
    let (_, _, params, _, _, _) =
        first_fn("fn f(string s: ==\"x\", bool b: !=true) -> int { return 1 }");
    assert_eq!(
        params[0].refinement.as_ref().expect("refinement").operand,
        RefineOperand::Str("x".to_string())
    );
    assert_eq!(
        params[1].refinement.as_ref().expect("refinement").operand,
        RefineOperand::Bool(true)
    );
}

#[test]
fn requires_single_with_message() {
    let (ast, _, _, _, requires, _) =
        first_fn("fn f(int a) -> int requires a > 0, \"positive\" { return a }");
    assert_eq!(requires.len(), 1);
    match &ast.exprs.get(requires[0].condition).kind {
        rl_ast::nodes::ExpressionKind::Binary { .. } => {}
        other => panic!("expected binary condition, got {other:?}"),
    }
    assert!(requires[0].message.is_some());
}

#[test]
fn requires_multiple_items() {
    let (_, _, _, _, requires, _) =
        first_fn("fn f(int a) -> int requires a > 0, \"pos\", a < 100 { return a }");
    assert_eq!(requires.len(), 2);
    assert!(requires[0].message.is_some());
    assert!(requires[1].message.is_none());
}

#[test]
fn ensures_without_message() {
    let (_, _, _, _, _, ensures) =
        first_fn("fn f(int a) -> int ensures ret <= a { return a }");
    assert_eq!(ensures.len(), 1);
    assert!(ensures[0].message.is_none());
}

#[test]
fn plain_fn_has_no_contracts() {
    let (_, _, params, return_type, requires, ensures) =
        first_fn("fn f(int a) -> int { return a }");
    assert!(params[0].refinement.is_none());
    assert!(requires.is_empty());
    assert!(ensures.is_empty());
    assert_eq!(return_type, TypeAnnotation::Int);
}

#[test]
fn test_params_still_default() {
    let source = "!#[test] fn f() { }";
    let (_, statements) = parse_fn(source);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { attribute, .. } => {
            assert_eq!(
                *attribute,
                Some(FunctionAttribute::Test(TestParams::default()))
            );
        }
        other => panic!("expected FunctionDeclaration, got {other:?}"),
    }
}
