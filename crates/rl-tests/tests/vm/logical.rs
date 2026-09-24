use crate::common;
use rl_vm::VmValue;

#[test]
fn logical_and_truth_table() {
    let result = common::compile_and_run(
        r#"
        dec bool a = true and true
        dec bool b = true and false
        dec bool c = false and true
        dec bool d = false and false
        a == true and b == false and c == false and d == false
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn logical_or_truth_table() {
    let result = common::compile_and_run(
        r#"
        dec bool a = true or true
        dec bool b = true or false
        dec bool c = false or true
        dec bool d = false or false
        a == true and b == true and c == true and d == false
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn logical_and_evaluates_right_side_when_left_is_true() {
    let err = common::compile_and_run(
        r#"
        dec bool x = true and (1 / 0 == 1)
        x
        "#,
    )
    .expect_err("right side of `and` should run and divide by zero");

    assert!(
        err.message().contains("division by zero"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn logical_or_evaluates_right_side_when_left_is_false() {
    let err = common::compile_and_run(
        r#"
        dec bool x = false or (1 / 0 == 1)
        x
        "#,
    )
    .expect_err("right side of `or` should run and divide by zero");

    assert!(
        err.message().contains("division by zero"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn logical_and_short_circuits_when_left_is_false() {
    let result = common::compile_and_run(
        r#"
        dec bool x = false and (1 / 0 == 1)
        x
        "#,
    )
    .expect("left `false` should short-circuit `and` without dividing by zero");

    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn logical_or_short_circuits_when_left_is_true() {
    let result = common::compile_and_run(
        r#"
        dec bool x = true or (1 / 0 == 1)
        x
        "#,
    )
    .expect("left `true` should short-circuit `or` without dividing by zero");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn logical_and_keeps_stack_balanced_around_dec() {
    let result = common::compile_and_run(
        r#"
        dec bool x = true and false
        dec int y = 42
        y
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn is_tests_primitives() {
    let result = common::compile_and_run(
        r#"
        dec bool a = 1 is int
        dec bool b = 1 is string
        dec bool c = 1.5 is float
        dec bool d = "hi" is string
        a and c and d and (b == false)
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_tests_containers_by_shape() {
    let result = common::compile_and_run(
        r#"
        dec bool a = [1] is arr[int]
        dec bool b = [1] is map[string, int]
        dec bool c = (1, 2) is (int, int, int)
        a and (b == false) and c
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_tests_nominal_types() {
    let result = common::compile_and_run(
        r#"
        record P { int x }
        tag C { Red, Blue }
        dec P p = P { x: 1 }
        dec bool a = p is P
        dec bool b = C.Red is C
        dec bool c = p is C
        a and b and (c == false)
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_tests_results() {
    let result = common::compile_and_run(
        r#"
        dec bool a = ok(1) is result[int]
        dec bool b = err("x") is result[int]
        dec bool c = ok(1) is int
        a and (b == false) and (c == false)
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_refines_branch_reads() {
    let result = common::compile_and_run(
        r#"
        dec any[int, string] x = 21
        dec int doubled = 0
        if x is int {
            doubled = x + x
        }
        doubled
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(42));
}
