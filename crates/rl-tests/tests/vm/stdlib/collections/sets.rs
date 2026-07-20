use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn set_add_to_empty() {
    let result = compile_and_run(
        r#"
get set_add, set_to_array from std::collections
dec set[int] s = {}
s = set_add(s, 42)?
dec arr[int] a = set_to_array(s)?
a
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Arr(Rc::new(vec![VmValue::Int(42)])));
}

#[test]
fn set_add_multiple() {
    let result = compile_and_run(
        r#"
get set_add, set_len from std::collections
dec set[int] s = {}
s = set_add(s, 1)?
s = set_add(s, 2)?
s = set_add(s, 3)?
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn set_add_duplicate_is_noop() {
    let result = compile_and_run(
        r#"
get set_add, set_len from std::collections
dec set[int] s = {}
s = set_add(s, 5)?
s = set_add(s, 5)?
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn set_add_with_existing_elements() {
    let result = compile_and_run(
        r#"
get set_add, set_len from std::collections
dec set[int] s = {10, 20, 30}
s = set_add(s, 40)?
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(4));
}

#[test]
fn set_remove_existing() {
    let has_1 = compile_and_run(
        r#"
get set_remove, set_contains from std::collections
dec set[int] s = {1, 2, 3}
s = set_remove(s, 2)?
dec bool has_1 = set_contains(s, 1)?
has_1
"#,
    )
    .unwrap();
    let has_2 = compile_and_run(
        r#"
get set_remove, set_contains from std::collections
dec set[int] s = {1, 2, 3}
s = set_remove(s, 2)?
dec bool has_2 = set_contains(s, 2)?
has_2
"#,
    )
    .unwrap();
    let has_3 = compile_and_run(
        r#"
get set_remove, set_contains from std::collections
dec set[int] s = {1, 2, 3}
s = set_remove(s, 2)?
dec bool has_3 = set_contains(s, 3)?
has_3
"#,
    )
    .unwrap();
    assert_eq!(has_1, VmValue::Bool(true));
    assert_eq!(has_2, VmValue::Bool(false));
    assert_eq!(has_3, VmValue::Bool(true));
}

#[test]
fn set_remove_nonexistent_is_noop() {
    let result = compile_and_run(
        r#"
get set_remove, set_len from std::collections
dec set[int] s = {1, 2, 3}
s = set_remove(s, 99)?
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn set_remove_from_empty() {
    let result = compile_and_run(
        r#"
get set_remove, set_len from std::collections
dec set[int] s = {}
s = set_remove(s, 1)?
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn set_remove_all_elements() {
    let result = compile_and_run(
        r#"
get set_remove, set_is_empty from std::collections
dec set[int] s = {1}
s = set_remove(s, 1)?
dec bool empty = set_is_empty(s)?
empty
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn set_contains_found() {
    let result = compile_and_run(
        r#"
get set_contains from std::collections
dec set[int] s = {10, 20, 30}
dec bool x = set_contains(s, 20)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn set_contains_not_found() {
    let result = compile_and_run(
        r#"
get set_contains from std::collections
dec set[int] s = {10, 20, 30}
dec bool x = set_contains(s, 99)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn set_contains_empty_set() {
    let result = compile_and_run(
        r#"
get set_contains from std::collections
dec set[int] s = {}
dec bool x = set_contains(s, 1)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn set_contains_strings() {
    let result = compile_and_run(
        r#"
get set_contains from std::collections
dec set[string] s = {"a", "b", "c"}
dec bool x = set_contains(s, "b")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn set_len_empty() {
    let result = compile_and_run(
        r#"
get set_len from std::collections
dec set[int] s = {}
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn set_len_nonempty() {
    let result = compile_and_run(
        r#"
get set_len from std::collections
dec set[int] s = {1, 2, 3, 4, 5}
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(5));
}

#[test]
fn set_is_empty_true() {
    let result = compile_and_run(
        r#"
get set_is_empty from std::collections
dec set[int] s = {}
dec bool x = set_is_empty(s)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn set_is_empty_false() {
    let result = compile_and_run(
        r#"
get set_is_empty from std::collections
dec set[int] s = {42}
dec bool x = set_is_empty(s)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn set_to_array_empty() {
    let result = compile_and_run(
        r#"
get set_to_array from std::collections
get len from std::array
dec set[int] s = {}
dec arr[int] a = set_to_array(s)?
dec int n = len(a)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn set_to_array_preserves_elements() {
    let result = compile_and_run(
        r#"
get set_to_array, set_len from std::collections
get arr_sort from std::array
dec set[int] s = {3, 1, 2}
dec arr[int] a = set_to_array(s)?
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    let items = compile_and_run(
        r#"
get set_to_array, set_len from std::collections
get arr_sort from std::array
dec set[int] s = {3, 1, 2}
dec arr[int] a = set_to_array(s)?
a = arr_sort(a)?
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));

    assert_eq!(
        items,
        VmValue::Arr(Rc::new(vec![
            VmValue::Int(1),
            VmValue::Int(2),
            VmValue::Int(3)
        ]))
    );
}

#[test]
fn set_with_booleans() {
    let result = compile_and_run(
        r#"
get set_add, set_len from std::collections
dec set[bool] s = {}
s = set_add(s, true)?
s = set_add(s, false)?
s = set_add(s, true)?
dec int n = set_len(s)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn set_add_remove_roundtrip() {
    let has_1 = compile_and_run(
        r#"
get set_add, set_remove, set_contains from std::collections
dec set[int] s = {}
s = set_add(s, 1)?
s = set_add(s, 2)?
s = set_add(s, 3)?
s = set_remove(s, 2)?
dec bool has_1 = set_contains(s, 1)?
has_1
"#,
    )
    .unwrap();
    let has_2 = compile_and_run(
        r#"
get set_add, set_remove, set_contains from std::collections
dec set[int] s = {}
s = set_add(s, 1)?
s = set_add(s, 2)?
s = set_add(s, 3)?
s = set_remove(s, 2)?
dec bool has_2 = set_contains(s, 2)?
has_2
"#,
    )
    .unwrap();
    let has_3 = compile_and_run(
        r#"
get set_add, set_remove, set_contains from std::collections
dec set[int] s = {}
s = set_add(s, 1)?
s = set_add(s, 2)?
s = set_add(s, 3)?
s = set_remove(s, 2)?
dec bool has_3 = set_contains(s, 3)?
has_3
"#,
    )
    .unwrap();
    assert_eq!(has_1, VmValue::Bool(true));
    assert_eq!(has_2, VmValue::Bool(false));
    assert_eq!(has_3, VmValue::Bool(true));
}
