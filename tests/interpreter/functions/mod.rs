use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn main_function_is_entry_point_when_present() {
    let evaluator = eval_program(
        r#"
dec int x = 0
fn main() {
    x = 7
}
x = 99
"#,
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn explicit_entry_function_overrides_main() {
    let evaluator = eval_program(
        r#"
dec int x = 0
fn main() {
    x = 1
}
!#[entry]
fn custom() {
    x = 2
}
"#,
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(2)));
}

#[test]
fn scripts_without_entry_still_run_top_level() {
    let evaluator = eval_program(
        r#"
dec int x = 0
x = 3
"#,
    )
    .unwrap();

    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(3)));
}
