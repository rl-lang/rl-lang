use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn dec_int() {
    let evaluator = match eval_program("dec int x = 42") {
        Ok(e) => e,
        Err(_) => panic!(),
    };
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn dec_float() {
    let evaluator = match eval_program("dec float x = 42") {
        Ok(e) => e,
        Err(_) => panic!(),
    };
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn int_undefined_variable_is_error() {
    assert!(eval_program("dec int x = y").is_err());
}

#[test]
fn float_undefined_variable_is_error() {
    assert!(eval_program("dec float x = y").is_err());
}
