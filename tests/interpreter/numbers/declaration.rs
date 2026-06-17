use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn dec_int() {
    let evaluator = eval_program("dec int x = 42").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn dec_float() {
    let evaluator = eval_program("dec float x = 42.0").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(42.0)));
}

#[test]
fn const_int() {
    let evaluator = eval_program("CONST int x = 42").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn const_float() {
    let evaluator = eval_program("CONST float x = 42.0").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(42.0)));
}

#[test]
fn assign_int() {
    let evaluator = eval_program("dec int x = 42\nx = 3").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn assign_float() {
    let evaluator = eval_program("dec float x = 42.0\nx = 3.0").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(3.0)));
}

#[test]
fn compound_assign_int() {
    let evaluator = eval_program("dec int x = 42\nx += 3").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(45)));
}

#[test]
fn compound_assign_float() {
    let evaluator = eval_program("dec float x = 42.0\nx += 3.0").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(45.0)));
}

#[test]
fn int_assigned_float_is_error() {
    assert!(eval_program("dec int x = 1.0").is_err());
}

#[test]
fn float_assigned_int_is_error() {
    assert!(eval_program("dec float x = 1").is_err());
}

#[test]
fn const_int_reassigned_is_error() {
    assert!(eval_program("CONST int x = 1\nx = 2").is_err());
}

#[test]
fn const_float_reassigned_is_error() {
    assert!(eval_program("CONST float x = 1.0\nx = 2.0").is_err());
}

#[test]
fn int_undefined_variable_is_error() {
    assert!(eval_program("dec int x = y").is_err());
}

#[test]
fn float_undefined_variable_is_error() {
    assert!(eval_program("dec float x = y").is_err());
}

#[test]
fn const_int_undefined_variable_is_error() {
    assert!(eval_program("CONST int x = y").is_err());
}

#[test]
fn const_float_undefined_variable_is_error() {
    assert!(eval_program("CONST float x = y").is_err());
}
