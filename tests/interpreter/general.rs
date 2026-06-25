use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

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
fn recursive_function() {
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
fn higher_order_function_via_lambda() {
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
fn integer_addition() {
    let ev = eval_program("dec int x = 10 + 5").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(15)));
}

#[test]
fn integer_subtraction() {
    let ev = eval_program("dec int x = 10 - 3").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn integer_multiplication() {
    let ev = eval_program("dec int x = 6 * 7").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn integer_division() {
    let ev = eval_program("dec int x = 10 / 2").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(5)));
}

// should add guard
#[test]
fn division_by_zero_is_error() {
    assert!(eval_program("dec int x = 1 / 0").is_err());
}

#[test]
fn float_arithmetic() {
    let ev = eval_program("dec float x = 1.5 + 2.5").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(4.0)));
}

#[test]
fn unary_negation() {
    let ev = eval_program("dec int x = -(5)").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(-5)));
}

#[test]
fn null_declaration() {
    let ev = eval_program("dec int x = null").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Null));
}

// should treat it as value
#[test]
fn null_equality() {
    let ev = eval_program("dec bool x = null == null").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}
