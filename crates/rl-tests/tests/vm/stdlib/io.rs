use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn read_rejects_too_many_args() {
    let result = compile_and_run(
        r#"
get read from std::io
get result_unwrap_err from std::res
dec result[string] r = read("a", "b")
dec string msg = result_unwrap_err(r)
msg
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str(Rc::from("read: expects 0 or 1 argument(s), got 2"))
    );
}

#[test]
fn read_int_rejects_too_many_args() {
    let result = compile_and_run(
        r#"
get read_int from std::io
get result_unwrap_err from std::res
dec result[int] r = read_int("a", "b")
dec string msg = result_unwrap_err(r)
msg
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str(Rc::from("read_int: expects 0 or 1 argument(s), got 2"))
    );
}

#[test]
fn read_float_rejects_too_many_args() {
    let result = compile_and_run(
        r#"
get read_float from std::io
get result_unwrap_err from std::res
dec result[float] r = read_float("a", "b")
dec string msg = result_unwrap_err(r)
msg
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str(Rc::from("read_float: expects 0 or 1 argument(s), got 2"))
    );
}
