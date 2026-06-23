use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn bit_and_int() {
    let evaluator = eval_program(
        "
        get bit_and from std::bitwise
        dec int x = bit_and(5, 3)
        ",
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(1)));
}

#[test]
fn bit_or_int() {
    let evaluator = eval_program(
        "
        get bit_or from std::bitwise
        dec int x = bit_or(5, 3)
        ",
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn bit_xor_int() {
    let evaluator = eval_program(
        "
        get bit_xor from std::bitwise
        dec int x = bit_xor(5, 3)
        ",
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(6)));
}

#[test]
fn bit_not_int() {
    let evaluator = eval_program(
        "
        get bit_not from std::bitwise
        dec int x = bit_not(0)
        ",
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(!0)));
}
