use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn dec_char() {
    let ev = eval_program("dec char x = 'a'").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Char('a')));
}

#[test]
fn const_char() {
    let ev = eval_program("CONST char x = 'z'").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Char('z')));
}

#[test]
fn char_reassign() {
    let ev = eval_program("dec char x = 'a'\nx = 'b'").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Char('b')));
}

#[test]
fn const_char_reassigned_is_error() {
    assert!(eval_program("CONST char x = 'a'\nx = 'b'").is_err());
}

#[test]
fn char_equality() {
    let ev = eval_program("dec bool x = 'a' == 'a'").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn char_inequality() {
    let ev = eval_program("dec bool x = 'a' != 'b'").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn char_to_string() {
    let ev = eval_program(
        r#"
get to_string from std::types
dec string x = to_string('q')?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::String("q".to_string())));
}

#[test]
fn int_to_char() {
    // ASCII 65 = 'A'
    let ev = eval_program(
        r#"
get to_char from std::types
dec char x = to_char(65)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Char('A')));
}

#[test]
fn is_char_true() {
    let ev = eval_program(
        r#"
get is_char from std::types
dec bool x = is_char('x')
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_char_false_for_int() {
    let ev = eval_program(
        r#"
get is_char from std::types
dec bool x = is_char(42)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}
