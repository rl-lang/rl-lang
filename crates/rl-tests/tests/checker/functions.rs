use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn undefined_call_errors() {
    assert_checker_msg("foo()", "undefined variable foo");
}

#[test]
fn user_fn_arity_errors() {
    assert_checker_msg(
        "fn f(int a) { }\nf(1, 2)",
        "function expects 1 argument(s), got 2",
    );
}

#[test]
fn user_fn_arg_type_errors() {
    assert_checker_msg(
        r#"fn f(int a) { }
f("hi")"#,
        "type mismatch: expected Int, got String",
    );
}

#[test]
fn non_callable_errors() {
    assert_checker_msg("dec int x = 5\nx(1)", "Int is not callable");
}

#[test]
fn unannotated_result_return_flows_through_propagate() {
    assert_checker_clean(
        "get read_file from std::fs\nfn grab(string p) { read_file(p) }\ndec s = grab(\"x\")?\n",
    );
}

#[test]
fn propagate_inside_unannotated_fn_is_allowed() {
    assert_checker_clean(
        "get read_file from std::fs\nfn grab(string p) { read_file(p)? }\ndec s = grab(\"x\")?\n",
    );
}

#[test]
fn propagate_inside_declared_non_result_still_errors() {
    assert_checker_msg(
        "get read_file from std::fs\nfn grab(string p) -> string { read_file(p)? }",
        "`?` cannot be used in a function that does not return a result",
    );
}

#[test]
fn handle_param_accepts_concrete_handle() {
    assert_checker_clean(
        "get open, close from std::fs\nfn shut(handle h) { close(h) }\ndec f = open(\"x\", \"r\")?\nshut(f)\n",
    );
}

#[test]
fn annotated_result_handle_return_flows() {
    assert_checker_clean(
        "get open, close from std::fs\nfn grab(string p) -> result[handle] { open(p, \"r\") }\ndec h = grab(\"x\")?\nclose(h)\n",
    );
}

#[test]
fn stdlib_typed_arity_errors() {
    assert_checker_msg(
        "get pow from std::math\npow(2)",
        "pow expects 2 argument(s), got 1",
    );
}

#[test]
fn propagate_non_result_errors() {
    assert_checker_msg(
        "dec int x = 5\nx?",
        "`?` operator requires a result, got Int",
    );
}

#[test]
fn lambda_passes() {
    assert_checker_clean(
        "get arr_map from std::array\narr_map([1, 2], fn(int x) -> int { return x })",
    );
}

#[test]
fn lambda_return_type_mismatch_errors() {
    assert_checker_msg(
        "get arr_map from std::array\narr_map([1, 2], fn(int x) -> string { return x })",
        "return type mismatch",
    );
}

#[test]
fn propagate_in_non_result_function_errors() {
    assert_checker_msg(
        r#"fn demo() -> string {
            dec result[string] r = err("oops")
            dec cleaned = r?
            return cleaned
        }"#,
        "`?` cannot be used in a function that does not return a result",
    );
}

#[test]
fn propagate_in_result_function_passes() {
    assert_checker_clean(
        r#"fn demo() -> result[string] {
            dec result[string] r = err("oops")
            dec cleaned = r?
            return ok(cleaned)
        }"#,
    );
}

#[test]
fn propagate_in_lambda_non_result_errors() {
    assert_checker_msg(
        r#"get arr_map from std::array
dec result[string] r = err("oops")
arr_map([1], fn(int x) -> string { dec cleaned = r? return cleaned })"#,
        "`?` cannot be used in a function that does not return a result",
    );
}
