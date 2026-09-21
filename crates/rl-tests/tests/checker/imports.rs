use crate::checker::common::{assert_checker_clean, assert_checker_msg};

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
