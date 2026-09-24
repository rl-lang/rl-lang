use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn zip_int_string() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
dec arr[int] a = [1, 2, 3]
dec arr[string] b = ["one", "two", "three"]
dec arr[(int, string)] z = arr_zip(a, b)
z
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![
            VmValue::Tuple(Rc::new(vec![
                VmValue::Int(1),
                VmValue::Str(Rc::from("one"))
            ])),
            VmValue::Tuple(Rc::new(vec![
                VmValue::Int(2),
                VmValue::Str(Rc::from("two"))
            ])),
            VmValue::Tuple(Rc::new(vec![
                VmValue::Int(3),
                VmValue::Str(Rc::from("three"))
            ])),
        ]))
    );
}

#[test]
fn zip_int_int() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
dec arr[int] a = [10, 20, 30]
dec arr[int] b = [1, 2, 3]
dec arr[(int, int)] z = arr_zip(a, b)
z
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![
            VmValue::Tuple(Rc::new(vec![VmValue::Int(10), VmValue::Int(1)])),
            VmValue::Tuple(Rc::new(vec![VmValue::Int(20), VmValue::Int(2)])),
            VmValue::Tuple(Rc::new(vec![VmValue::Int(30), VmValue::Int(3)])),
        ]))
    );
}

#[test]
fn zip_truncates_to_shorter_left() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
get len from std
dec arr[int] a = [1, 2]
dec arr[int] b = [10, 20, 30, 40]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn zip_truncates_to_shorter_right() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
get len from std
dec arr[int] a = [1, 2, 3, 4]
dec arr[int] b = [10, 20]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn zip_empty_left() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
get len from std
dec arr[int] a = []
dec arr[int] b = [1, 2, 3]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn zip_empty_right() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
get len from std
dec arr[int] a = [1, 2, 3]
dec arr[int] b = []
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn zip_both_empty() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
get len from std
dec arr[int] a = []
dec arr[int] b = []
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn zip_single_element() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
dec arr[int] a = [42]
dec arr[string] b = ["hi"]
dec arr[(int, string)] z = arr_zip(a, b)
z
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![VmValue::Tuple(Rc::new(vec![
            VmValue::Int(42),
            VmValue::Str(Rc::from("hi")),
        ]))]))
    );
}

#[test]
fn zip_then_map() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
get len from std
dec arr[int] a = [1, 2, 3]
dec arr[int] b = [4, 5, 6]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn zip_non_array_first_arg_errors() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
dec arr[(int, int)] z = arr_zip(42, [1, 2])
"#,
    );
    assert!(result.is_err());
}

#[test]
fn zip_non_array_second_arg_errors() {
    let result = compile_and_run(
        r#"
get arr_zip from std::array
dec arr[(int, int)] z = arr_zip([1, 2], "oops")
"#,
    );
    assert!(result.is_err());
}
