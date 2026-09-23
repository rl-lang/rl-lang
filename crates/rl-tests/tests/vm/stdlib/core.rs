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
get len from std::array
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
vals
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![VmValue::Int(2), VmValue::Int(1)]))
    );
}
