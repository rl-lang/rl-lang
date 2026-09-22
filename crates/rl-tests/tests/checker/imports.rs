use crate::checker::common::{assert_checker_clean, assert_checker_msg};
use crate::common::check;

fn assert_checker_warns(source: &str, fragment: &str) {
    let checker = check(source);
    assert!(
        checker.errors.is_empty(),
        "expected no errors, got: {:?}",
        checker.errors.iter().map(|e| e.message().to_string()).collect::<Vec<_>>()
    );
    let warnings: Vec<String> = checker
        .warnings
        .iter()
        .map(|e| e.message().to_string())
        .collect();
    assert!(
        warnings.iter().any(|m| m.contains(fragment)),
        "expected a warning containing {fragment:?}, got: {warnings:?}"
    );
}

fn assert_no_warnings(source: &str) {
    let checker = check(source);
    let warnings: Vec<String> = checker
        .warnings
        .iter()
        .map(|e| e.message().to_string())
        .collect();
    assert!(warnings.is_empty(), "expected no warnings, got: {warnings:?}");
}

#[test]
fn unimported_bare_call_errors() {
    assert_checker_msg(r#"println("hello")"#, "import it before use");
}

#[test]
fn imported_bare_call_passes() {
    assert_checker_clean("get println from std::io\nprintln(\"hello\")");
}

#[test]
fn fully_qualified_path_passes_without_import() {
    assert_checker_clean(r#"std::io::println("hello")"#);
}

#[test]
fn import_unknown_module_errors() {
    assert_checker_msg("get x from std::nope", "unknown module");
}

#[test]
fn import_unknown_name_errors() {
    assert_checker_msg("get nope from std::io", "not defined");
}

#[test]
fn user_function_shadows_unimported_stdlib_bare_call() {
    assert_checker_clean("fn println(string s) {\n}\nprintln(\"hello\")");
}

#[test]
fn nested_module_import_passes() {
    assert_checker_clean("get is_inf from std::math::consts");
}

#[test]
fn fully_qualified_nested_path_passes() {
    assert_checker_clean("std::math::consts::is_inf(1.0)");
}

#[test]
fn wildcard_import_passes() {
    assert_checker_clean("get * from std::math\nabs(-5)");
}

#[test]
fn aliased_import_passes() {
    assert_checker_clean("get sin as sine from std::math\nsine(0.0)");
}

#[test]
fn mixed_alias_and_plain_passes() {
    assert_checker_clean("get sin as sine, cos from std::math\nsine(0.0)\ncos(0.0)");
}

#[test]
fn deprecated_bare_call_warns() {
    assert_checker_warns(
        "get read_file from std::io\nread_file(\"x\")",
        "'std::io::read_file' is deprecated: use std::fs::read_file instead",
    );
}

#[test]
fn deprecated_qualified_call_warns() {
    assert_checker_warns(
        "std::io::read_file(\"x\")",
        "'std::io::read_file' is deprecated",
    );
}

#[test]
fn deprecated_alias_warns_with_canonical_path() {
    assert_checker_warns(
        "get read_file as rf from std::io\nrf(\"x\")",
        "'std::io::read_file' is deprecated",
    );
}

#[test]
fn deprecated_wildcard_warns() {
    assert_checker_warns(
        "get * from std::io\nread_file(\"x\")",
        "'std::io::read_file' is deprecated",
    );
}

#[test]
fn deprecated_method_call_warns() {
    assert_checker_warns(
        "get read_file from std::io\ndec string p = \"x\"\nread_file(p)",
        "'std::io::read_file' is deprecated",
    );
}

#[test]
fn deprecated_allow_suppresses() {
    let checker = crate::common::check(
        "get read_file from std::io\n!#[allow(deprecated)]\nfn demo() -> null {\n    read_file(\"x\")\n}",
    );
    let warnings: Vec<String> = checker
        .warnings
        .iter()
        .map(|e| e.message().to_string())
        .collect();
    assert!(
        !warnings.iter().any(|m| m.contains("deprecated")),
        "expected no deprecation warnings, got: {warnings:?}"
    );
}

#[test]
fn non_deprecated_import_silent() {
    assert_no_warnings("get println from std::io\nprintln(\"hi\")");
}

#[test]
fn deprecated_std_rl_warns() {
    assert_checker_warns(
        "get eval from std::rl\neval(\"1\")",
        "'std::rl::eval' is deprecated",
    );
}

#[test]
fn deprecated_path_fn_warns() {
    assert_checker_warns(
        "get path_exists from std::path\npath_exists(\"x\")",
        "'std::path::path_exists' is deprecated",
    );
}
