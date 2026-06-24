use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn dec_bool_true() {
    let ev = eval_program("dec bool x = true").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn dec_bool_false() {
    let ev = eval_program("dec bool x = false").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn const_bool() {
    let ev = eval_program("CONST bool x = true").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn bool_reassign() {
    let ev = eval_program("dec bool x = true\nx = false").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn const_bool_reassigned_is_error() {
    assert!(eval_program("CONST bool x = true\nx = false").is_err());
}

/*
 * keeping them for when `and` and `or` implemented
#[test]
fn logical_and_true() {
    let ev = eval_program("dec bool x = true and true").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn logical_and_false() {
    let ev = eval_program("dec bool x = true and false").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn logical_or_true() {
    let ev = eval_program("dec bool x = false or true").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn logical_or_false() {
    let ev = eval_program("dec bool x = false or false").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}
*/

#[test]
fn logical_not_true() {
    let ev = eval_program("dec bool x = !false").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn logical_not_false() {
    let ev = eval_program("dec bool x = !true").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn eq_ints() {
    let ev = eval_program("dec bool x = 1 == 1").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn neq_ints() {
    let ev = eval_program("dec bool x = 1 != 2").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn lt_ints() {
    let ev = eval_program("dec bool x = 1 < 2").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn gt_ints() {
    let ev = eval_program("dec bool x = 2 > 1").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn lte_ints() {
    let ev = eval_program("dec bool x = 2 <= 2").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn gte_ints() {
    let ev = eval_program("dec bool x = 3 >= 2").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn eq_strings() {
    let ev = eval_program(r#"dec bool x = "hello" == "hello""#).unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn neq_strings() {
    let ev = eval_program(r#"dec bool x = "hello" != "world""#).unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn if_true_branch() {
    let ev = eval_program(
        r#"
dec int x = 0
if true {
    x = 1
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(1)));
}

#[test]
fn if_false_skips_body() {
    let ev = eval_program(
        r#"
dec int x = 0
if false {
    x = 1
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(0)));
}

#[test]
fn if_else_false_branch() {
    let ev = eval_program(
        r#"
dec int x = 0
if false {
    x = 1
} else {
    x = 2
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(2)));
}

#[test]
fn if_elif_branch() {
    let ev = eval_program(
        r#"
dec int x = 0
if false {
    x = 1
} else if true {
    x = 2
} else {
    x = 3
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(2)));
}
