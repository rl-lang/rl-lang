use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn arr_new_push_get_set() {
    let result = compile_and_run(
        r#"
get __arr_new, __arr_push, __arr_get, __arr_set from core
dec a = __arr_push(__arr_push(__arr_new(), 10), 20)
dec b = __arr_set(a, 0, 99)
__arr_get(b, 0) + __arr_get(b, 1)
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(119));
}

#[test]
fn arr_get_out_of_bounds_aborts() {
    let result = compile_and_run(
        r#"
get __arr_get from core
__arr_get([1], 5)
"#,
    );
    assert!(result.is_err());
}

#[test]
fn map_new_set_get_keys() {
    let result = compile_and_run(
        r#"
get __map_new, __map_set, __map_get, __map_keys from core
get len from std::array
get result_unwrap from std::res
dec m = __map_new()
__map_set(m, "a", 1)
__map_set(m, "b", 2)
dec int x = __map_get(m, "a")
dec int n = result_unwrap(len(__map_keys(m)))
x + n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn map_get_missing_aborts() {
    let result = compile_and_run(
        r#"
get __map_new, __map_get from core
dec m = __map_new()
__map_get(m, "nope")
"#,
    );
    assert!(result.is_err());
}

#[test]
fn set_new_add_has() {
    let result = compile_and_run(
        r#"
get __set_new, __set_add, __set_has from core
dec s = __set_new()
__set_add(s, 1)
__set_add(s, 2)
dec bool a = __set_has(s, 1)
dec bool b = __set_has(s, 9)
a
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    let result = compile_and_run(
        r#"
get __set_new, __set_add, __set_has from core
dec s = __set_new()
__set_add(s, 1)
__set_has(s, 9)
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn lengths_and_strings() {
    let result = compile_and_run(
        r#"
get __arr_len, __map_len, __set_len from core
get __str_len, __str_get_byte, __str_slice, __str_concat from core
get __set_new, __set_add from core
dec s = __set_new()
__set_add(s, 1)
dec int total = __arr_len([1, 2, 3]) + __map_len({"a": 1}) + __set_len(s)
dec int blen = __str_len("hi")
dec byte b = __str_get_byte("hi", 1)
dec string sub = __str_slice("hello", 1, 4)
dec string cat = __str_concat("a", "b")
total + blen
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn str_slice_rejects_bad_range() {
    let result = compile_and_run(
        r#"
get __str_slice from core
__str_slice("hi", 0, 5)
"#,
    );
    assert!(result.is_err());
}

#[test]
fn syscall_getpid() {
    let result = compile_and_run(
        r#"
get __syscall6 from core
get os_name from std::process
dec int pid = 0
if os_name() == "linux" {
    pid = __syscall6(39, 0, 0, 0, 0, 0, 0)
}
pid
"#,
    )
    .unwrap();
    match result {
        VmValue::Int(pid) => assert!(pid > 0),
        other => panic!("expected int pid, got {:?}", other),
    }
}

#[test]
fn removes_abort_on_absent() {
    let result = compile_and_run(
        r#"
get __arr_remove, __map_remove, __set_remove from core
get __map_new, __map_set, __map_has, __map_keys, __set_new, __set_add from core
get len from std::array
get result_unwrap from std::res
dec a = __arr_remove([10, 20, 30], 1)
dec m = __map_new()
__map_set(m, "a", 1)
dec bool present = __map_has(m, "a")
dec bool gone = __map_has(m, "nope")
__map_remove(m, "a")
dec s = __set_new()
__set_add(s, 1)
__set_remove(s, 1)
dec int total = result_unwrap(len(a)) + result_unwrap(len(__map_keys(m)))
dec bool good = present and !gone and total == 2
good
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn remove_missing_aborts() {
    let result = compile_and_run(
        r#"
get __map_new, __map_remove from core
dec m = __map_new()
__map_remove(m, "never-there")
"#,
    );
    assert!(result.is_err());
    let result = compile_and_run(
        r#"
get __set_new, __set_remove from core
dec s = __set_new()
__set_remove(s, 1)
"#,
    );
    assert!(result.is_err());
}

#[test]
fn abort_fails_loud() {
    let result = compile_and_run(
        r#"
get __abort from core
__abort("boom")
"#,
    );
    let err = result.unwrap_err();
    assert!(err.message().contains("boom"));
}

#[test]
fn result_ok_value_unwraps() {
    let result = compile_and_run(
        r#"
get __result_ok_value from core
__result_ok_value(ok(41)) + 1
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn result_ok_value_aborts_on_err() {
    let result = compile_and_run(
        r#"
get __result_ok_value from core
__result_ok_value(err("boom"))
"#,
    );
    let err = result.unwrap_err();
    assert!(err.message().contains("__result_ok_value"));
}

#[test]
fn result_err_value_unwraps() {
    let result = compile_and_run(
        r#"
get __result_err_value from core
__result_err_value(err("boom"))
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str("boom".into()));
}

#[test]
fn result_err_value_aborts_on_ok() {
    let result = compile_and_run(
        r#"
get __result_err_value from core
__result_err_value(ok(1))
"#,
    );
    let err = result.unwrap_err();
    assert!(err.message().contains("__result_err_value"));
}

#[test]
fn type_of_names_types() {
    let result = compile_and_run(
        r#"
get __type_of from core
__type_of(42)
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str("int".into()));
}

// An RL-written stdlib function using only core:: primitives plus
// operators: values of a map, in key order. This is the Phase A
// pattern (logic in RL, primitives from core).
#[test]
fn rl_written_values_fn() {
    let result = compile_and_run(
        r#"
get __map_keys, __map_get, __arr_push from core
get len, arr_contains from std::array
get result_unwrap from std::res

fn my_values(map[string, int] m) -> arr[int] {
    dec keys = __map_keys(m)
    dec arr[int] out = []
    dec int i = 0
    dec int n = result_unwrap(len(keys))
    while i < n {
        out = __arr_push(out, __map_get(m, keys[i]))
        i = i + 1
    }
    return out
}

dec vals = my_values({"b": 2, "a": 1})
dec bool has1 = result_unwrap(arr_contains(vals, 1))
dec bool has2 = result_unwrap(arr_contains(vals, 2))
dec int n = result_unwrap(len(vals))
has1
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}
