use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn to_upper() {
    let result = compile_and_run(
        r#"
get to_upper from std::str
dec string x = to_upper("hello")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("HELLO")));
}

#[test]
fn to_lower() {
    let result = compile_and_run(
        r#"
get to_lower from std::str
dec string x = to_lower("WORLD")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("world")));
}

#[test]
fn trim_whitespace() {
    let result = compile_and_run(
        r#"
get trim from std::str
dec string x = trim("  hello  ")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("hello")));
}

#[test]
fn trim_start() {
    let result = compile_and_run(
        r#"
get trim_start from std::str
dec string x = trim_start("  hello  ")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("hello  ")));
}

#[test]
fn trim_end() {
    let result = compile_and_run(
        r#"
get trim_end from std::str
dec string x = trim_end("  hello  ")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("  hello")));
}

#[test]
fn contains_true() {
    let result = compile_and_run(
        r#"
get contains from std::str
dec bool x = contains("hello world", "world")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn contains_false() {
    let result = compile_and_run(
        r#"
get contains from std::str
dec bool x = contains("hello world", "foo")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn starts_with_true() {
    let result = compile_and_run(
        r#"
get starts_with from std::str
dec bool x = starts_with("hello", "he")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn ends_with_true() {
    let result = compile_and_run(
        r#"
get ends_with from std::str
dec bool x = ends_with("hello", "lo")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn replace_str() {
    let result = compile_and_run(
        r#"
get replace from std::str
dec string x = replace("foo bar foo", "foo", "baz")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("baz bar baz")));
}

#[test]
fn repeat_str() {
    let result = compile_and_run(
        r#"
get repeat from std::str
dec string x = repeat("ab", 3)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("ababab")));
}

#[test]
fn is_empty_true() {
    let result = compile_and_run(
        r#"
get is_empty from std::str
dec bool x = is_empty("")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_empty_false() {
    let result = compile_and_run(
        r#"
get is_empty from std::str
dec bool x = is_empty("hi")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn concat_strings() {
    let result = compile_and_run(
        r#"
get concat from std::str
dec string x = concat("foo", "bar")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("foobar")));
}

#[test]
fn char_at() {
    let result = compile_and_run(
        r#"
get char_at from std::str
dec char x = char_at("hello", 1)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Char('e'));
}

#[test]
fn slice_str() {
    let result = compile_and_run(
        r#"
get slice from std::str
dec string x = slice("hello world", 6, 11)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("world")));
}

#[test]
fn pad_left() {
    let result = compile_and_run(
        r#"
get pad_left from std::str
dec string x = pad_left("hi", 5, ' ')
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("   hi")));
}

#[test]
fn pad_right() {
    let result = compile_and_run(
        r#"
get pad_right from std::str
dec string x = pad_right("hi", 5, '.')
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("hi...")));
}

#[test]
fn replace_substring() {
    let result = compile_and_run(
        r#"
get replace from std::str
dec string x = replace("hello world", "world", "rl")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("hello rl")));
}

#[test]
fn split_by_delimiter() {
    let result = compile_and_run(
        r#"
get split from std::str
get arr_count from std::array
dec arr[string] parts = split("a,b,c", ",")
dec int n = arr_count(parts)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn repeat_string() {
    let result = compile_and_run(
        r#"
get repeat from std::str
dec string x = repeat("ab", 3)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("ababab")));
}

#[test]
fn reverse_string() {
    let result = compile_and_run(
        r#"
get reverse from std::str
dec string x = reverse("hello")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("olleh")));
}

#[test]
fn str_len() {
    let result = compile_and_run(
        r#"
get len from std
dec int x = len("hello")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(5));
}

#[test]
fn index_of_found() {
    let result = compile_and_run(
        r#"
get index_of from std::str
dec int x = index_of("hello", "ll")
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn slice_string() {
    let result = compile_and_run(
        r#"
get slice from std::str
dec string x = slice("hello world", 6, 11)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("world")));
}
