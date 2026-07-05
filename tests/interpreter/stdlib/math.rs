use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn abs_positive() {
    let ev = eval_program(
        r#"
get abs from std::math
dec int x = abs(5)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(5)));
}

#[test]
fn abs_negative() {
    let ev = eval_program(
        r#"
get abs from std::math
dec int x = abs(-7)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn max_returns_larger() {
    let ev = eval_program(
        r#"
get max from std::math
dec int x = max(3, 9)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(9)));
}

#[test]
fn min_returns_smaller() {
    let ev = eval_program(
        r#"
get min from std::math
dec int x = min(3, 9)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn pow_integer() {
    let ev = eval_program(
        r#"
get pow from std::math
dec int x = pow(2, 10)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(1024)));
}

#[test]
fn sqrt_float() {
    let ev = eval_program(
        r#"
get sqrt from std::math
dec float x = sqrt(9.0)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(3.0)));
}

#[test]
fn floor_float() {
    let ev = eval_program(
        r#"
get floor from std::math
dec float x = floor(3.9)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(3.0)));
}

#[test]
fn ceil_float() {
    let ev = eval_program(
        r#"
get ceil from std::math
dec float x = ceil(3.1)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(4.0)));
}

#[test]
fn round_float_up() {
    let ev = eval_program(
        r#"
get round from std::math
dec float x = round(2.6)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(3.0)));
}

#[test]
fn round_float_down() {
    let ev = eval_program(
        r#"
get round from std::math
dec float x = round(2.4)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(2.0)));
}

#[test]
fn clamp_within_range() {
    let ev = eval_program(
        r#"
get clamp from std::math
dec int x = clamp(5, 0, 10)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(5)));
}

#[test]
fn clamp_below_min() {
    let ev = eval_program(
        r#"
get clamp from std::math
dec int x = clamp(-5, 0, 10)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(0)));
}

#[test]
fn clamp_above_max() {
    let ev = eval_program(
        r#"
get clamp from std::math
dec int x = clamp(99, 0, 10)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(10)));
}

#[test]
fn mod_operation() {
    let ev = eval_program(
        r#"
get mod from std::math
dec int x = mod(10, 3)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(1)));
}

#[test]
fn log2_eight() {
    let ev = eval_program(
        r#"
get log2 from std::math
dec float x = log2(8.0)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(3.0)));
}

#[test]
fn log10_thousand() {
    let ev = eval_program(
        r#"
get log10 from std::math
dec float x = log10(1000.0)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Float(3.0)));
}
