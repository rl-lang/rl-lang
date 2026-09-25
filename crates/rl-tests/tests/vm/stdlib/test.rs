use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn asserts_return_null() {
    let result = compile_and_run(
        r#"
get test_assert_eq, test_assert_ne from std::test
test_assert_eq(1 + 1, 2, "math")
test_assert_ne(1, 2, "different")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Null);
}

#[test]
fn skip_if_false_continues() {
    let result = compile_and_run(
        r#"
get test_skip_if from std::test
test_skip_if(false, "never")
42
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn skip_at_top_level_raises() {
    let result = compile_and_run(
        r#"
get test_skip from std::test
test_skip("no reason")
"#,
    );
    assert!(result.is_err());
}

#[test]
fn non_callable_panics_check_raises() {
    let result = compile_and_run(
        r#"
get test_assert_panics from std::test
test_assert_panics(42)
"#,
    );
    assert!(result.is_err());
}

#[test]
fn panics_check_observes_failure() {
    let result = compile_and_run(
        r#"
get test_assert_panics, test_assert_no_panic from std::test
get result_unwrap from std::res
test_assert_panics(fn() {
    result_unwrap(err("boom"))
})
test_assert_no_panic(fn() {
    1 + 1
})
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Null);
}

#[test]
fn lookup_runs_registered_case() {
    let result = compile_and_run(
        r#"
get test_assert_eq, test_run_registered from std::test

!#[test(register("math"))]
fn addition() {
    test_assert_eq(1 + 1, 2, "math")
}

test_run_registered("math")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn lookup_counts_failures() {
    let result = compile_and_run(
        r#"
get test_assert_eq, test_run_registered from std::test

!#[test(register("m"))]
fn bad() {
    test_assert_eq(1, 2, "boom")
}

test_run_registered("m")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn lookup_unknown_registry_raises() {
    let result = compile_and_run(
        r#"
get test_run_registered from std::test
test_run_registered("nope")
"#,
    );
    assert!(result.is_err());
}

#[test]
fn lookup_runs_setup_around_case() {
    let result = compile_and_run(
        r#"
get test_assert_eq, test_run_registered from std::test

dec int calls = 0

!#[setup]
fn before() {
    calls = calls + 1
}

!#[test(register("m"))]
fn check() {
    test_assert_eq(calls, 1, "setup ran")
}

test_run_registered("m")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}
