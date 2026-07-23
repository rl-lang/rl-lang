use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn map_contains_found() {
    let result = compile_and_run(
        r#"
get map_contains from std::collections
dec map[string, int] m = {"a": 1, "b": 2}
dec bool x = map_contains(m, "a")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn map_contains_not_found() {
    let result = compile_and_run(
        r#"
get map_contains from std::collections
dec map[string, int] m = {"a": 1, "b": 2}
dec bool x = map_contains(m, "z")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn map_contains_empty_map() {
    let result = compile_and_run(
        r#"
get map_contains from std::collections
dec map[string, int] m = {}
dec bool x = map_contains(m, "a")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn map_remove_existing() {
    let has_a = compile_and_run(
        r#"
get map_remove, map_contains from std::collections
dec map[string, int] m = {"a": 1, "b": 2}
m = map_remove(m, "a")?
dec bool has_a = map_contains(m, "a")?
has_a
"#,
    )
    .unwrap();
    let has_b = compile_and_run(
        r#"
get map_remove, map_contains from std::collections
dec map[string, int] m = {"a": 1, "b": 2}
m = map_remove(m, "a")?
dec bool has_b = map_contains(m, "b")?
has_b
"#,
    )
    .unwrap();
    assert_eq!(has_a, VmValue::Bool(false));
    assert_eq!(has_b, VmValue::Bool(true));
}

#[test]
fn map_remove_nonexistent_is_noop() {
    let result = compile_and_run(
        r#"
get map_remove, map_len from std::collections
dec map[string, int] m = {"a": 1}
m = map_remove(m, "z")?
dec int n = map_len(m)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn map_len_empty() {
    let result = compile_and_run(
        r#"
get map_len from std::collections
dec map[string, int] m = {}
dec int n = map_len(m)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn map_len_nonempty() {
    let result = compile_and_run(
        r#"
get map_len from std::collections
dec map[string, int] m = {"a": 1, "b": 2, "c": 3}
dec int n = map_len(m)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn map_is_empty_true() {
    let result = compile_and_run(
        r#"
get map_is_empty from std::collections
dec map[string, int] m = {}
dec bool x = map_is_empty(m)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn map_is_empty_false() {
    let result = compile_and_run(
        r#"
get map_is_empty from std::collections
dec map[string, int] m = {"a": 1}
dec bool x = map_is_empty(m)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn map_to_array_single_entry() {
    let result = compile_and_run(
        r#"
get map_to_array from std::collections
dec map[string, int] m = {"a": 1}
dec arr[(string, int)] a = map_to_array(m)?
a
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![VmValue::Tuple(Rc::new(vec![
            VmValue::Str(Rc::from("a")),
            VmValue::Int(1),
        ]))]))
    );
}

#[test]
fn map_to_array_empty() {
    let result = compile_and_run(
        r#"
get map_to_array from std::collections
get len from std::array
dec map[string, int] m = {}
dec arr[(string, int)] a = map_to_array(m)?
dec int n = len(a)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn map_get_found() {
    let result = compile_and_run(
        r#"
get map_get from std::collections
dec map[string, int] m = {"a": 1}
dec int x = map_get(m, "a")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn map_values_preserves_elements() {
    let result = compile_and_run(
        r#"
get map_values, map_len from std::collections
dec map[string, int] m = {"a": 1, "b": 2, "c": 3}
dec arr[int] values = map_values(m)?
dec int n = map_len(m)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn map_clear_empties_map() {
    let result = compile_and_run(
        r#"
get map_clear, map_is_empty from std::collections
dec map[string, int] m = {"a": 1, "b": 2}
m = map_clear(m)?
dec bool empty = map_is_empty(m)?
empty
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn map_merge_combines_keys() {
    let result = compile_and_run(
        r#"
get map_merge, map_len from std::collections
dec map[string, int] a = {"x": 1}
dec map[string, int] b = {"y": 2}
a = map_merge(a, b)?
dec int n = map_len(a)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn map_merge_overwrites_matching_keys() {
    let result = compile_and_run(
        r#"
get map_merge, map_get from std::collections
dec map[string, int] a = {"x": 1}
dec map[string, int] b = {"x": 9}
a = map_merge(a, b)?
dec int x = map_get(a, "x")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(9));
}
