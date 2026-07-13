use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn is_ok_on_ok() {
    let ev = eval_program(
        r#"
get is_ok from std::res
dec bool x = is_ok(ok(42))?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_ok_on_err() {
    let ev = eval_program(
        r#"
get is_ok from std::res
dec bool x = is_ok(err("oops"))?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn is_err_on_err() {
    let ev = eval_program(
        r#"
get is_err from std::res
dec bool x = is_err(err("oops"))?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn is_err_on_ok() {
    let ev = eval_program(
        r#"
get is_err from std::res
dec bool x = is_err(ok(42))?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn result_unwrap_ok() {
    let ev = eval_program(
        r#"
get result_unwrap from std::res
dec int x = result_unwrap(ok(99))
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(99)));
}

#[test]
fn result_unwrap_panics_on_err() {
    let result = eval_program(
        r#"
get result_unwrap from std::res
dec int x = result_unwrap(err("boom"))
"#,
    );
    assert!(result.is_err());
}

#[test]
fn result_unwrap_err_on_err() {
    let ev = eval_program(
        r#"
get result_unwrap_err from std::res
dec string x = result_unwrap_err(err("failed"))
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("failed".to_string()))
    );
}

#[test]
fn result_unwrap_err_panics_on_ok() {
    let result = eval_program(
        r#"
get result_unwrap_err from std::res
dec int x = result_unwrap_err(ok(42))
"#,
    );
    assert!(result.is_err());
}

#[test]
fn result_unwrap_or_ok_returns_inner() {
    let ev = eval_program(
        r#"
get result_unwrap_or from std::res
dec int x = result_unwrap_or(ok(7), -1)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn result_unwrap_or_err_returns_default() {
    let ev = eval_program(
        r#"
get result_unwrap_or from std::res
dec int x = result_unwrap_or(err("gone"), -1)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(-1)));
}

#[test]
fn result_map_transforms_ok() {
    let ev = eval_program(
        r#"
get result_map, result_unwrap from std::res
dec result[int] r = result_map(ok(5), fn(int n) -> int { return n * 2 })
dec int x = result_unwrap(r)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(10)));
}

#[test]
fn result_map_passes_through_err() {
    let ev = eval_program(
        r#"
get result_map, is_err from std::res
dec result[int] r = result_map(err("dead"), fn(int n) -> int { return n * 2 })
dec bool x = is_err(r)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn result_map_err_transforms_err() {
    let ev = eval_program(
        r#"
get result_map_err, result_unwrap_err from std::res
get concat from std::str
dec result[int] r = result_map_err(err("oops"), fn(string s) -> string { return concat("ERR: ", s) })
dec string x = result_unwrap_err(r)
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::String("ERR: oops".to_string()))
    );
}

#[test]
fn result_map_err_passes_through_ok() {
    let ev = eval_program(
        r#"
get result_map_err, result_unwrap from std::res
dec result[int] r = result_map_err(ok(42), fn(string s) -> string { return s })
dec int x = result_unwrap(r)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn function_returns_ok() {
    let ev = eval_program(
        r#"
get is_ok, result_unwrap from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err("division by zero")
    }
    return ok(a / b)
}
dec result[int] r = safe_div(10, 2)
dec bool ok_flag = is_ok(r)?
dec int val = result_unwrap(r)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("ok_flag"), Some(Value::Bool(true)));
    assert_eq!(ev.get_value_raw("val"), Some(Value::Integer(5)));
}

#[test]
fn function_returns_err() {
    let ev = eval_program(
        r#"
get is_err, result_unwrap_err from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err("division by zero")
    }
    return ok(a / b)
}
dec result[int] r = safe_div(10, 0)
dec bool err_flag = is_err(r)?
dec string msg = result_unwrap_err(r)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("err_flag"), Some(Value::Bool(true)));
    assert_eq!(
        ev.get_value_raw("msg"),
        Some(Value::String("division by zero".to_string()))
    );
}

#[test]
fn chained_result_map() {
    let ev = eval_program(
        r#"
get result_map, result_unwrap from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err("division by zero")
    }
    return ok(a / b)
}
dec result[int] r = result_map(safe_div(20, 4), fn(int n) -> int { return n + 1 })
dec int x = result_unwrap(r)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(6)));
}

#[test]
fn err_carries_int() {
    let ev = eval_program(
        r#"
get result_unwrap_err from std::res
dec result[int] r = err(404)
dec int code = result_unwrap_err(r)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("code"), Some(Value::Integer(404)));
}

#[test]
fn err_carries_bool() {
    let ev = eval_program(
        r#"
get result_unwrap_err from std::res
dec result[int] r = err(false)
dec bool b = result_unwrap_err(r)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("b"), Some(Value::Bool(false)));
}
