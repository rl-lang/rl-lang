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

#[test]
fn contracts_reject_non_bool_requires() {
    assert_checker_msg(
        "fn f(int x) -> int requires x { return x }",
        "must be bool",
    );
}

#[test]
fn contracts_reject_non_bool_ensures() {
    assert_checker_msg(
        "fn f(int x) -> int ensures x { return x }",
        "must be bool",
    );
}

#[test]
fn contracts_ret_unbound_outside_ensures() {
    assert_checker_msg(
        "fn f(int x) -> int requires ret > 0 { return x }",
        "undefined variable",
    );
}

#[test]
fn contract_refinement_violation_returns_err() {
    let result = crate::common::compile_and_run(
        r#"
fn withdraw(int amt: >0, int balance) -> result[int] {
    return ok(balance - amt)
}
get is_err from std::res
is_err(withdraw(0, 100))
"#,
    )
    .unwrap();
    assert_eq!(result, rl_vm::VmValue::Bool(true));
}

#[test]
fn contract_refinement_passes() {
    let result = crate::common::compile_and_run(
        r#"
fn withdraw(int amt: >0, int balance) -> result[int] {
    return ok(balance - amt)
}
withdraw(30, 100)?
"#,
    )
    .unwrap();
    assert_eq!(result, rl_vm::VmValue::Int(70));
}

#[test]
fn contract_bare_violation_aborts() {
    let result = crate::common::compile_and_run(
        r#"
fn bare(int x: >0) -> int {
    return x
}
bare(0)
"#,
    );
    assert!(result.is_err());
}

#[test]
fn contract_ensures_violation_returns_err() {
    let result = crate::common::compile_and_run(
        r#"
fn over(int x) -> result[int]
    ensures ret < 10
{
    return ok(x)
}
get is_err from std::res
is_err(over(50))
"#,
    )
    .unwrap();
    assert_eq!(result, rl_vm::VmValue::Bool(true));
}

#[test]
fn contract_ensures_skipped_on_err() {
    let result = crate::common::compile_and_run(
        r#"
fn maybe(bool b) -> result[int]
    ensures ret > 0
{
    if (b) { return err(0) }
    return ok(5)
}
get is_err from std::res
dec result[int] a = maybe(true)
dec int b = maybe(false)?
dec bool e = is_err(a)
e
"#,
    )
    .unwrap();
    assert_eq!(result, rl_vm::VmValue::Bool(true));
}

#[test]
fn contract_custom_message_surfaces() {
    let result = crate::common::compile_and_run(
        r#"
fn m(int a) -> result[int]
    requires a > 0, "pos", a < 100, "small"
{
    return ok(a)
}
get result_unwrap_err from std::res
result_unwrap_err(m(500))
"#,
    )
    .unwrap();
    assert_eq!(result, rl_vm::VmValue::Str("small".into()));
}

#[test]
fn contracts_proven_refinement_violation_errors() {
    assert_checker_msg(
        "fn withdraw(int amt: >0, int balance) -> result[int] {\n    return ok(balance - amt)\n}\nwithdraw(0, 100)",
        "contract violation (proven at compile time): refinement failed: amt > 0",
    );
}

#[test]
fn contracts_proven_requires_violation_uses_message() {
    assert_checker_msg(
        "fn withdraw(int amt, int balance) -> result[int]\n    requires amt > 0, \"positive\"\n{\n    return ok(balance - amt)\n}\nwithdraw(0, 100)",
        "contract violation (proven at compile time): positive",
    );
}

#[test]
fn contracts_proven_cross_param_violation_errors() {
    assert_checker_msg(
        "fn withdraw(int amt, int balance: >=amt) -> result[int] {\n    return ok(balance - amt)\n}\nwithdraw(50, 10)",
        "refinement failed: balance >= amt",
    );
}

#[test]
fn contracts_satisfied_call_is_clean() {
    assert_checker_clean(
        "fn withdraw(int amt: >0, int balance) -> result[int] {\n    return ok(balance - amt)\n}\nwithdraw(30, 100)",
    );
}

#[test]
fn contracts_dynamic_args_stay_silent() {
    assert_checker_clean(
        "fn withdraw(int amt: >0, int balance) -> result[int] {\n    return ok(balance - amt)\n}\ndec int n = 0\nwithdraw(n, 100)",
    );
}
