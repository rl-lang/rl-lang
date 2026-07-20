use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn set_add_to_empty() {
    let ev = eval_program(
        r#"
get set_add, set_to_array from std::set
dec set[int] s = {}
s = set_add(s, 42)?
dec arr[int] a = set_to_array(s)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("a"),
        Some(Value::Values {
            items_type: rl_ast::statements::TypeAnnotation::Int,
            items: vec![Value::Integer(42)],
        })
    );
}

#[test]
fn set_add_multiple() {
    let ev = eval_program(
        r#"
get set_add, set_len from std::set
dec set[int] s = {}
s = set_add(s, 1)?
s = set_add(s, 2)?
s = set_add(s, 3)?
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(3)));
}

#[test]
fn set_add_duplicate_is_noop() {
    let ev = eval_program(
        r#"
get set_add, set_len from std::set
dec set[int] s = {}
s = set_add(s, 5)?
s = set_add(s, 5)?
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(1)));
}

#[test]
fn set_add_with_existing_elements() {
    let ev = eval_program(
        r#"
get set_add, set_len from std::set
dec set[int] s = {10, 20, 30}
s = set_add(s, 40)?
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(4)));
}

#[test]
fn set_add_type_mismatch() {
    let ev = eval_program(
        r#"
get set_add from std::set
dec set[int] s = {1, 2}
s = set_add(s, "hello")?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("s"), Some(Value::Null));
}

#[test]
fn set_remove_existing() {
    let ev = eval_program(
        r#"
get set_remove, set_contains from std::set
dec set[int] s = {1, 2, 3}
s = set_remove(s, 2)?
dec bool has_1 = set_contains(s, 1)?
dec bool has_2 = set_contains(s, 2)?
dec bool has_3 = set_contains(s, 3)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("has_1"), Some(Value::Bool(true)));
    assert_eq!(ev.get_value_raw("has_2"), Some(Value::Bool(false)));
    assert_eq!(ev.get_value_raw("has_3"), Some(Value::Bool(true)));
}

#[test]
fn set_remove_nonexistent_is_noop() {
    let ev = eval_program(
        r#"
get set_remove, set_len from std::set
dec set[int] s = {1, 2, 3}
s = set_remove(s, 99)?
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(3)));
}

#[test]
fn set_remove_from_empty() {
    let ev = eval_program(
        r#"
get set_remove, set_len from std::set
dec set[int] s = {}
s = set_remove(s, 1)?
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(0)));
}

#[test]
fn set_remove_all_elements() {
    let ev = eval_program(
        r#"
get set_remove, set_is_empty from std::set
dec set[int] s = {1}
s = set_remove(s, 1)?
dec bool empty = set_is_empty(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("empty"), Some(Value::Bool(true)));
}

#[test]
fn set_contains_found() {
    let ev = eval_program(
        r#"
get set_contains from std::set
dec set[int] s = {10, 20, 30}
dec bool x = set_contains(s, 20)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn set_contains_not_found() {
    let ev = eval_program(
        r#"
get set_contains from std::set
dec set[int] s = {10, 20, 30}
dec bool x = set_contains(s, 99)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn set_contains_empty_set() {
    let ev = eval_program(
        r#"
get set_contains from std::set
dec set[int] s = {}
dec bool x = set_contains(s, 1)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn set_contains_strings() {
    let ev = eval_program(
        r#"
get set_contains from std::set
dec set[string] s = {"a", "b", "c"}
dec bool x = set_contains(s, "b")?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn set_len_empty() {
    let ev = eval_program(
        r#"
get set_len from std::set
dec set[int] s = {}
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(0)));
}

#[test]
fn set_len_nonempty() {
    let ev = eval_program(
        r#"
get set_len from std::set
dec set[int] s = {1, 2, 3, 4, 5}
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(5)));
}

#[test]
fn set_is_empty_true() {
    let ev = eval_program(
        r#"
get set_is_empty from std::set
dec set[int] s = {}
dec bool x = set_is_empty(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn set_is_empty_false() {
    let ev = eval_program(
        r#"
get set_is_empty from std::set
dec set[int] s = {42}
dec bool x = set_is_empty(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn set_to_array_empty() {
    let ev = eval_program(
        r#"
get set_to_array from std::set
get len from std::array
dec set[int] s = {}
dec arr[int] a = set_to_array(s)?
dec int n = len(a)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(0)));
}

#[test]
fn set_to_array_preserves_elements() {
    let ev = eval_program(
        r#"
get set_to_array, set_len from std::set
get arr_sort from std::array
dec set[int] s = {3, 1, 2}
dec arr[int] a = set_to_array(s)?
a = arr_sort(a)?
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(3)));
    let a = ev.get_value_raw("a").unwrap();
    let items = match a {
        Value::Values { items, .. } => items.clone(),
        other => panic!("expected array, got {:?}", other),
    };
    assert_eq!(
        items,
        vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]
    );
}

#[test]
fn set_to_array_type_mismatch() {
    let result = eval_program(
        r#"
get set_to_array from std::set
dec set[int] s = {1, 2}
dec arr[string] a = set_to_array(s)?
"#,
    );
    assert!(result.is_err());
}

#[test]
fn set_with_booleans() {
    let ev = eval_program(
        r#"
get set_add, set_len from std::set
dec set[bool] s = {}
s = set_add(s, true)?
s = set_add(s, false)?
s = set_add(s, true)?
dec int n = set_len(s)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(2)));
}

#[test]
fn set_add_remove_roundtrip() {
    let ev = eval_program(
        r#"
get set_add, set_remove, set_contains from std::set
dec set[int] s = {}
s = set_add(s, 1)?
s = set_add(s, 2)?
s = set_add(s, 3)?
s = set_remove(s, 2)?
dec bool has_1 = set_contains(s, 1)?
dec bool has_2 = set_contains(s, 2)?
dec bool has_3 = set_contains(s, 3)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("has_1"), Some(Value::Bool(true)));
    assert_eq!(ev.get_value_raw("has_2"), Some(Value::Bool(false)));
    assert_eq!(ev.get_value_raw("has_3"), Some(Value::Bool(true)));
}
