use crate::common::eval_program;
use rl_interpreter::values::Value;

// -- tuple declaration --

#[test]
fn tuple_declaration() {
    let ev = eval_program("dec (int, string) t = (42, \"hello\")").unwrap();
    assert_eq!(
        ev.get_value_raw("t"),
        Some(Value::Tuple(vec![
            Value::Integer(42),
            Value::String("hello".into())
        ]))
    );
}

#[test]
fn tuple_index_int() {
    let ev = eval_program(
        r#"
dec (int, string) t = (42, "hello")
dec int x = t[0]
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn tuple_index_string() {
    let ev = eval_program(
        r#"
dec (int, string) t = (42, "hello")
dec string s = t[1]
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("s"), Some(Value::String("hello".into())));
}

#[test]
fn tuple_index_out_of_bounds() {
    assert!(
        eval_program(
            r#"
dec (int, string) t = (42, "hello")
dec int x = t[5]
"#
        )
        .is_err()
    );
}

#[test]
fn tuple_three_elements() {
    let ev = eval_program(
        r#"
dec (int, float, bool) t = (1, 4.14, true)
dec int a = t[0]
dec float b = t[1]
dec bool c = t[2]
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("a"), Some(Value::Integer(1)));
    assert_eq!(ev.get_value_raw("b"), Some(Value::Float(4.14)));
    assert_eq!(ev.get_value_raw("c"), Some(Value::Bool(true)));
}

#[test]
fn tuple_len() {
    let ev = eval_program(
        r#"
get len from std::array
dec (int, string, bool) t = (1, "hi", false)
dec int n = len(t)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(3)));
}

// -- destructure --

#[test]
fn destructure_two_bindings() {
    let ev = eval_program(
        r#"
dec int x, string y = (10, "world")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(10)));
    assert_eq!(ev.get_value_raw("y"), Some(Value::String("world".into())));
}

#[test]
fn destructure_three_bindings() {
    let ev = eval_program(
        r#"
dec int a, float b, bool c = (5, 2.5, true)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("a"), Some(Value::Integer(5)));
    assert_eq!(ev.get_value_raw("b"), Some(Value::Float(2.5)));
    assert_eq!(ev.get_value_raw("c"), Some(Value::Bool(true)));
}

#[test]
fn destructure_arity_mismatch() {
    assert!(eval_program("dec int x, string y = (1, \"hi\", true)").is_err());
}

// -- error type --

#[test]
fn error_wraps_int() {
    let ev = eval_program("dec error e = error(404)").unwrap();
    assert_eq!(
        ev.get_value_raw("e"),
        Some(Value::Error(Box::new(Value::Integer(404))))
    );
}

#[test]
fn error_wraps_string() {
    let ev = eval_program(r#"dec error e = error("not found")"#).unwrap();
    assert_eq!(
        ev.get_value_raw("e"),
        Some(Value::Error(Box::new(Value::String("not found".into()))))
    );
}

#[test]
fn error_cannot_wrap_error() {
    assert!(eval_program("dec error e = error(error(1))").is_err());
}

#[test]
fn is_error_true() {
    let ev = eval_program(
        r#"
get is_error from std::types
dec error e = error(1)
dec bool b = is_error(e)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("b"), Some(Value::Bool(true)));
}

#[test]
fn is_error_false() {
    let ev = eval_program(
        r#"
get is_error from std::types
dec int x = 42
dec bool b = is_error(x)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("b"), Some(Value::Bool(false)));
}

#[test]
fn error_unwrap_value() {
    let ev = eval_program(
        r#"
get error_unwrap from std::types
dec error e = error(99)
dec int x = error_unwrap(e)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(99)));
}

#[test]
fn error_unwrap_non_error_fails() {
    assert!(
        eval_program(
            r#"
get error_unwrap from std::types
dec int x = 5
dec int y = error_unwrap(x)
"#
        )
        .is_err()
    );
}
