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
fn bit_not_byte() {
    let evaluator = eval_program(
        "
        get bit_not from std::bitwise
        dec int x = bit_not(0)
        ",
    )
    .unwrap();

    assert_eq!(
        evaluator.get_value_raw("x"),
        Some(Value::Integer(!0u8 as i64))
    );
}

#[test]
fn bit_shift_left_int() {
    let evaluator = eval_program(
        "
        get bit_shift_left from std::bitwise
        dec int x = bit_shift_left(5, 1)
        ",
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(10)));
}

#[test]
fn bit_shift_right_int() {
    let evaluator = eval_program(
        "
        get bit_shift_right from std::bitwise
        dec int x = bit_shift_right(10, 1)
        ",
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(5)));
}

#[test]
fn count_bits_int() {
    let evaluator = eval_program(
        "
        get count_bits from std::bitwise
        dec int x = count_bits(7)
        ",
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn leading_zeros_int() {
    let evaluator = eval_program(
        "
        get leading_zeros from std::bitwise
        dec int x = leading_zeros(8)
        ",
    )
    .unwrap();

    assert_eq!(
        evaluator.get_value_raw("x"),
        Some(Value::Integer(8u8.leading_zeros() as i64))
    );
}

#[test]
fn trailing_zeros_int() {
    let evaluator = eval_program(
        "
        get trailing_zeros from std::bitwise
        dec int x = trailing_zeros(8)
        ",
    )
    .unwrap();

    assert_eq!(
        evaluator.get_value_raw("x"),
        Some(Value::Integer(8u8.trailing_zeros() as i64))
    );
}
