use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn to_upper() {
    let ev = eval_program(
        r#"
get to_upper from std::str
dec string x = to_upper("hello")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("HELLO".to_string()))
    );
}

#[test]
fn to_lower() {
    let ev = eval_program(
        r#"
get to_lower from std::str
dec string x = to_lower("WORLD")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("world".to_string()))
    );
}

#[test]
fn trim_whitespace() {
    let ev = eval_program(
        r#"
get trim from std::str
dec string x = trim("  hello  ")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("hello".to_string()))
    );
}

#[test]
fn trim_start() {
    let ev = eval_program(
        r#"
get trim_start from std::str
dec string x = trim_start("  hello  ")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("hello  ".to_string()))
    );
}

#[test]
fn trim_end() {
    let ev = eval_program(
        r#"
get trim_end from std::str
dec string x = trim_end("  hello  ")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("  hello".to_string()))
    );
}

#[test]
fn contains_true() {
    let ev = eval_program(
        r#"
get contains from std::str
dec bool x = contains("hello world", "world")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn contains_false() {
    let ev = eval_program(
        r#"
get contains from std::str
dec bool x = contains("hello world", "foo")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn starts_with_true() {
    let ev = eval_program(
        r#"
get starts_with from std::str
dec bool x = starts_with("hello", "he")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn ends_with_true() {
    let ev = eval_program(
        r#"
get ends_with from std::str
dec bool x = ends_with("hello", "lo")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn replace_str() {
    let ev = eval_program(
        r#"
get replace from std::str
dec string x = replace("foo bar foo", "foo", "baz")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("baz bar baz".to_string()))
    );
}

#[test]
fn repeat_str() {
    let ev = eval_program(
        r#"
get repeat from std::str
dec string x = repeat("ab", 3)
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("ababab".to_string()))
    );
}

#[test]
fn is_empty_true() {
    let ev = eval_program(
        r#"
get is_empty from std::str
dec bool x = is_empty("")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_empty_false() {
    let ev = eval_program(
        r#"
get is_empty from std::str
dec bool x = is_empty("hi")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn concat_strings() {
    let ev = eval_program(
        r#"
get concat from std::str
dec string x = concat("foo", "bar")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("foobar".to_string()))
    );
}

#[test]
fn char_at() {
    let ev = eval_program(
        r#"
get char_at from std::str
dec char x = char_at("hello", 1)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Char('e')));
}

#[test]
fn slice_str() {
    let ev = eval_program(
        r#"
get slice from std::str
dec string x = slice("hello world", 6, 11)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("world".to_string()))
    );
}

#[test]
fn pad_left() {
    let ev = eval_program(
        r#"
get pad_left from std::str
dec string x = pad_left("hi", 5, ' ')
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("   hi".to_string()))
    );
}

#[test]
fn pad_right() {
    let ev = eval_program(
        r#"
get pad_right from std::str
dec string x = pad_right("hi", 5, '.')
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("hi...".to_string()))
    );
}

#[test]
fn replace_substring() {
    let ev = eval_program(
        r#"
get replace from std::str
dec string x = replace("hello world", "world", "rl")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("hello rl".to_string()))
    );
}

#[test]
fn split_by_delimiter() {
    let ev = eval_program(
        r#"
get split from std::str
get arr_count from std::array
dec arr[string] parts = split("a,b,c", ",")
dec int n = arr_count(parts)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(3)));
}

#[test]
fn repeat_string() {
    let ev = eval_program(
        r#"
get repeat from std::str
dec string x = repeat("ab", 3)
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("ababab".to_string()))
    );
}

#[test]
fn reverse_string() {
    let ev = eval_program(
        r#"
get reverse from std::str
dec string x = reverse("hello")
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("olleh".to_string()))
    );
}

#[test]
fn str_len() {
    let ev = eval_program(
        r#"
get len from std::array
dec int x = len("hello")"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(5)));
}

#[test]
fn index_of_found() {
    let ev = eval_program(
        r#"
get index_of from std::str
dec int x = index_of("hello", "ll")
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(2)));
}

#[test]
fn slice_string() {
    let ev = eval_program(
        r#"
get slice from std::str
dec string x = slice("hello world", 6, 11)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("world".to_string()))
    );
}
