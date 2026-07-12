use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn main_function_is_entry_point_when_present() {
    let ev = eval_program(
        r#"
dec int x = 0
fn main() {
    x = 7
}
x = 99
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn explicit_entry_function_overrides_main() {
    let ev = eval_program(
        r#"
dec int x = 0
fn main() {
    x = 1
}
!#[entry]
fn custom() {
    x = 2
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(2)));
}

#[test]
fn scripts_without_entry_still_run_top_level() {
    let ev = eval_program(
        r#"
dec int x = 0
x = 3
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn function_returns_value() {
    let ev = eval_program(
        r#"
fn add(int a, int b) -> int {
    return a + b
}
dec int x = add(3, 4)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn recursive_factorial() {
    let ev = eval_program(
        r#"
fn fact(int n) -> int {
    if n <= 1 {
        return 1
    }
    return n * fact(n - 1)
}
dec int x = fact(5)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(120)));
}

#[test]
fn recursive_fibonacci() {
    let ev = eval_program(
        r#"
fn fib(int n) -> int {
    if n <= 1 {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}
dec int x = fib(10)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(55)));
}

#[test]
fn higher_order_function() {
    let ev = eval_program(
        r#"
fn apply(fn f, int x) -> int {
    return f(x)
}
dec int x = apply(fn(int n) -> int { return n * 2 }, 7)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(14)));
}

#[test]
fn lambda_assigned_to_variable() {
    let ev = eval_program(
        r#"
dec fn double = fn(int n) -> int { return n * 2 }
dec int x = double(5)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(10)));
}

#[test]
fn closure_captures_outer_variable() {
    let ev = eval_program(
        r#"
dec int base = 10
dec fn adder = fn(int n) -> int { return n + base }
dec int x = adder(5)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(15)));
}

#[test]
fn function_mutates_global() {
    let ev = eval_program(
        r#"
dec int counter = 0
fn increment() {
    counter += 1
}
increment()
increment()
increment()
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("counter"), Some(Value::Integer(3)));
}

#[test]
fn function_with_no_return_is_null() {
    let ev = eval_program(
        r#"
fn nothing() {
}
dec int x = 0
nothing()
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(0)));
}

#[test]
fn early_return_from_function() {
    let ev = eval_program(
        r#"
fn first_positive(int a, int b, int c) -> int {
    if a > 0 {
        return a
    }
    if b > 0 {
        return b
    }
    return c
}
dec int x = first_positive(-1, -2, 42)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn multiple_return_paths() {
    let ev = eval_program(
        r#"
fn sign(int n) -> int {
    if n > 0 {
        return 1
    } else if n < 0 {
        return -1
    } else {
        return 0
    }
}
dec int a = sign(10)
dec int b = sign(-5)
dec int c = sign(0)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("a"), Some(Value::Integer(1)));
    assert_eq!(ev.get_value_raw("b"), Some(Value::Integer(-1)));
    assert_eq!(ev.get_value_raw("c"), Some(Value::Integer(0)));
}

#[test]
fn function_called_in_expression() {
    let ev = eval_program(
        r#"
fn double(int n) -> int { return n * 2 }
dec int x = double(3) + double(4)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(14)));
}
