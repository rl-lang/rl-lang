use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn float_to_int() {
    let ev = eval_program(
        r#"
get to_int from std::types
dec int x = to_int(3.9)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn string_to_int() {
    let ev = eval_program(
        r#"
get to_int from std::types
dec int x = to_int("42")?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn bool_true_to_int() {
    let ev = eval_program(
        r#"
get to_int from std::types
dec int x = to_int(true)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(1)));
}

#[test]
fn bool_false_to_int() {
    let ev = eval_program(
        r#"
get to_int from std::types
dec int x = to_int(false)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(0)));
}

#[test]
fn int_to_float() {
    let ev = eval_program(
        r#"
get to_float from std::types
dec float x = to_float(5)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(5.0)));
}

#[test]
fn string_to_float() {
    let ev = eval_program(
        r#"
get to_float from std::types
dec float x = to_float("4.14")?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(4.14)));
}

#[test]
fn int_to_string() {
    let ev = eval_program(
        r#"
get to_string from std::types
dec string x = to_string(99)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::String("99".to_string())));
}

#[test]
fn bool_to_string() {
    let ev = eval_program(
        r#"
get to_string from std::types
dec string x = to_string(true)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("true".to_string()))
    );
}

#[test]
fn float_to_string() {
    let ev = eval_program(
        r#"
get to_string from std::types
dec string x = to_string(1.5)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("1.5".to_string()))
    );
}

#[test]
fn one_to_bool_true() {
    let ev = eval_program(
        r#"
get to_bool from std::types
dec bool x = to_bool(1)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn zero_to_bool_false() {
    let ev = eval_program(
        r#"
get to_bool from std::types
dec bool x = to_bool(0)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn is_int_true() {
    let ev = eval_program(
        r#"
get is_int from std::types
dec bool x = is_int(42)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_int_false_for_float() {
    let ev = eval_program(
        r#"
get is_int from std::types
dec bool x = is_int(1.0)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn is_float_true() {
    let ev = eval_program(
        r#"
get is_float from std::types
dec bool x = is_float(3.14)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_string_true() {
    let ev = eval_program(
        r#"
get is_string from std::types
dec bool x = is_string("hi")?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_bool_true() {
    let ev = eval_program(
        r#"
get is_bool from std::types
dec bool x = is_bool(false)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_null_true() {
    let ev = eval_program(
        r#"
get is_null from std::types
dec bool x = is_null(null)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_null_false_for_int() {
    let ev = eval_program(
        r#"
get is_null from std::types
dec bool x = is_null(0)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn to_hex() {
    let ev = eval_program(
        r#"
get to_hex from std::types
dec string x = to_hex(255)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::String("ff".to_string())));
}

#[test]
fn to_bin() {
    let ev = eval_program(
        r#"
get to_bin from std::types
dec string x = to_bin(5)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("101".to_string()))
    );
}

#[test]
fn to_oct() {
    let ev = eval_program(
        r#"
get to_oct from std::types
dec string x = to_oct(8)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::String("10".to_string())));
}

#[test]
fn to_bool_from_true_string() {
    let ev = eval_program(
        r#"
get to_bool from std::types
dec bool x = to_bool("true")?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn to_bool_from_int_one() {
    let ev = eval_program(
        r#"
get to_bool from std::types
dec bool x = to_bool(1)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn to_bool_from_int_zero() {
    let ev = eval_program(
        r#"
get to_bool from std::types
dec bool x = to_bool(0)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn to_string_from_int() {
    let ev = eval_program(
        r#"
get to_string from std::types
dec string x = to_string(42)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::String("42".to_string())));
}

#[test]
fn to_string_from_float() {
    let ev = eval_program(
        r#"
get to_string from std::types
dec string x = to_string(3.14)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("3.14".to_string()))
    );
}

#[test]
fn to_string_from_bool() {
    let ev = eval_program(
        r#"
get to_string from std::types
dec string x = to_string(true)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("true".to_string()))
    );
}

#[test]
fn to_hex_from_int() {
    let ev = eval_program(
        r#"
get to_hex from std::types
dec string x = to_hex(255)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::String("ff".to_string())));
}

#[test]
fn to_bin_from_int() {
    let ev = eval_program(
        r#"
get to_bin from std::types
dec string x = to_bin(5)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("101".to_string()))
    );
}

#[test]
fn to_oct_from_int() {
    let ev = eval_program(
        r#"
get to_oct from std::types
dec string x = to_oct(8)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::String("10".to_string())));
}

#[test]
fn is_int_false() {
    let ev = eval_program(
        r#"
get is_int from std::types
dec bool x = is_int("hello")?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn is_null_false() {
    let ev = eval_program(
        r#"
get is_null from std::types
dec bool x = is_null(42)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}
