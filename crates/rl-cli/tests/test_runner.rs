#![cfg(feature = "vm")]

use rl_cli::pipeline;
use rl_utils::source::SourceFile;

fn run_source(source_text: &str, match_pattern: Option<&str>) -> pipeline::test::TestReport {
    let file = SourceFile::new("test_input.rl", source_text.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) =
        rl_parser::parser_logic::Parser::parse(tokens, file.clone()).expect("parse failed");

    let checker_tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (checker_ast, checker_stmts) =
        rl_parser::parser_logic::Parser::parse(checker_tokens, file.clone())
            .expect("parse failed");
    let mut checker = rl_checker::TypeChecker::new().with_ast_arena(checker_ast);
    let errors = checker.check(&checker_stmts);
    assert!(errors.is_empty(), "type check errors: {:?}", errors);

    let (arena, resolved) = pipeline::vm::resolve(&file, ast, stmts);
    pipeline::test::run_tests(&file, &arena, &resolved, match_pattern)
}

#[test]
fn discovers_and_runs_passing_cases() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

!#[test]
fn addition() {
    test_assert_eq(1 + 1, 2, "math")
}

!#[test(group("strings"))]
fn concat() {
    test_assert_eq("a", "a", "same")
}
"#,
        None,
    );
    assert_eq!(report.ran, 2);
    assert_eq!(report.ok, 2);
    assert_eq!(report.failed, 0);
}

#[test]
fn failing_case_reports_message() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

!#[test]
fn bad() {
    test_assert_eq(1, 2, "boom")
}
"#,
        None,
    );
    assert_eq!(report.ran, 1);
    assert_eq!(report.failed, 1);
    assert!(report.failures.iter().any(|m| m.contains("boom")));
}

#[test]
fn skip_does_not_fail() {
    let report = run_source(
        r#"
get test_skip from std::test

!#[test]
fn later() {
    test_skip("not yet")
}
"#,
        None,
    );
    assert_eq!(report.ran, 1);
    assert_eq!(report.skipped, 1);
    assert_eq!(report.failed, 0);
}

#[test]
fn match_filters_by_group() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

!#[test(group("a"))]
fn first() {
    test_assert_eq(1, 1, "ok")
}

!#[test(group("b"))]
fn second() {
    test_assert_eq(1, 1, "ok")
}
"#,
        Some("b"),
    );
    assert_eq!(report.ran, 1);
    assert_eq!(report.ok, 1);
}

#[test]
fn setup_runs_before_each_case() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

dec int calls = 0

!#[setup]
fn before() {
    calls = calls + 1
}

!#[test]
fn one() {
    test_assert_eq(calls, 1, "first")
}

!#[test]
fn two() {
    test_assert_eq(calls, 2, "second")
}
"#,
        None,
    );
    assert_eq!(report.failed, 0);
    assert_eq!(report.ok, 2);
}

#[test]
fn entry_body_never_runs() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

!#[entry]
fn main() {
    test_assert_eq(1, 2, "entry must not run")
}

!#[test]
fn fine() {
    test_assert_eq(1, 1, "ok")
}
"#,
        None,
    );
    assert_eq!(report.ran, 1);
    assert_eq!(report.failed, 0);
}

#[test]
fn property_passes_over_generated_inputs() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

!#[test(cases(20))]
fn squares(int x) {
    test_assert_eq(x * x >= 0, true, "square")
}
"#,
        None,
    );
    assert_eq!(report.ran, 1);
    assert_eq!(report.ok, 1);
    assert_eq!(report.failed, 0);
}

#[test]
fn property_failure_reports_shrunk_inputs() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

!#[test(cases(50))]
fn bounded(int x: >0) {
    test_assert_eq(x < 32, true, "small")
}
"#,
        None,
    );
    assert_eq!(report.ran, 1);
    assert_eq!(report.failed, 1);
    assert!(report.failures.iter().any(|m| m.contains("inputs:")));
}

#[test]
fn property_unsupported_type_fails_cleanly() {
    let report = run_source(
        r#"
!#[test(cases(5))]
fn maps(map[string, int] m) {
}
"#,
        None,
    );
    assert_eq!(report.failed, 1);
    assert!(report.failures.iter().any(|m| m.contains("cannot generate")));
}

#[test]
fn match_filters_by_register() {
    let report = run_source(
        r#"
get test_assert_eq from std::test

!#[test(register("fast"))]
fn one() {
    test_assert_eq(1, 1, "ok")
}

!#[test(register("slow"))]
fn two() {
    test_assert_eq(1, 1, "ok")
}
"#,
        Some("slow"),
    );
    assert_eq!(report.ran, 1);
    assert_eq!(report.ok, 1);
}
