use rl_interpreter::values::Value;

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

#[test]
fn break_exits_while() {
    let ev = eval_program(
        r#"
dec int x = 0
while true {
    x += 1
    if x == 3 {
        break
    }
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn break_exits_for_range() {
    let ev = eval_program(
        r#"
dec int x = 0
for i in 0..100 {
    if i == 5 {
        break
    }
    x = i
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(4)));
}

#[test]
fn continue_skips_iteration() {
    let ev = eval_program(
        r#"
dec int total = 0
for i in 0..6 {
    if i == 3 {
        continue
    }
    total += i
}
"#,
    )
    .unwrap();
    // 0+1+2+4+5 = 12 (skips 3)
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(12)));
}

#[test]
fn continue_in_while() {
    let ev = eval_program(
        r#"
dec int i = 0
dec int total = 0
while i < 5 {
    i += 1
    if i == 3 {
        continue
    }
    total += i
}
"#,
    )
    .unwrap();
    // 1+2+4+5 = 12 (skips 3)
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(12)));
}

#[test]
fn for_each_over_array() {
    let ev = eval_program(
        r#"
dec arr[int] items = [10, 20, 30]
dec int total = 0
for item in items {
    total += item
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(60)));
}

#[test]
fn for_each_empty_array_skips_body() {
    let ev = eval_program(
        r#"
dec arr[int] items = []
dec int x = 0
for item in items {
    x = 99
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(0)));
}

#[test]
fn nested_for_ranges() {
    let ev = eval_program(
        r#"
dec int total = 0
for i in 0..3 {
    for j in 0..3 {
        total += 1
    }
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(9)));
}

#[test]
fn if_else_if_else_chain() {
    let ev = eval_program(
        r#"
dec int x = 5
dec string res = "none"
if x < 0 {
    res = "negative"
} else if x == 0 {
    res = "zero"
} else if x < 10 {
    res = "small"
} else {
    res = "large"
}
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("res"),
        Some(Value::String("small".to_string()))
    );
}

#[test]
fn nested_if_in_while() {
    let ev = eval_program(
        r#"
dec int x = 0
dec int evens = 0
while x < 10 {
    x += 1
    if x == 0 {
        evens += 1
    } else if x == 2 {
        evens += 1
    } else if x == 4 {
        evens += 1
    } else if x == 6 {
        evens += 1
    } else if x == 8 {
        evens += 1
    } else if x == 10 {
        evens += 1
    }
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("evens"), Some(Value::Integer(5)));
}

#[test]
fn for_c_style_loop() {
    let ev = eval_program(
        r#"
dec int total = 0
for [int i = 0, i < 5, i += 1] {
    total += i
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(10)));
}
