use rl_lang::interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn while_counts_up() {
    let ev = eval_program(
        r#"
dec int x = 0
while x < 5 {
    x += 1
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(5)));
}

#[test]
fn while_false_body_skipped() {
    let ev = eval_program(
        r#"
dec int x = 0
while false {
    x = 99
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(0)));
}

#[test]
fn for_range_counts() {
    let ev = eval_program(
        r#"
dec int total = 0
for i in 0..5 {
    total += i
}
"#,
    )
    .unwrap();
    // 0+1+2+3+4 = 10
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(10)));
}

#[test]
fn for_range_exclusive_upper_bound() {
    let ev = eval_program(
        r#"
dec int last = -1
for i in 0..3 {
    last = i
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("last"), Some(Value::Integer(2)));
}
