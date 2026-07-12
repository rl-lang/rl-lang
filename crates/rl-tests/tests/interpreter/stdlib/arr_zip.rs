use std::rc::Rc;

use rl_ast::statements::TypeAnnotation;
use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn zip_int_string() {
    let ev = eval_program(
        r#"
get arr_zip from std::array
dec arr[int] a = [1, 2, 3]
dec arr[string] b = ["one", "two", "three"]
dec arr[(int, string)] z = arr_zip(a, b)
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("z"),
        Some(Value::Values {
            items_type: TypeAnnotation::Tuple(Rc::new(vec![
                TypeAnnotation::Null,
                TypeAnnotation::Null,
            ])),
            items: vec![
                Value::Tuple(vec![Value::Integer(1), Value::String("one".to_string())]),
                Value::Tuple(vec![Value::Integer(2), Value::String("two".to_string())]),
                Value::Tuple(vec![Value::Integer(3), Value::String("three".to_string())]),
            ],
        })
    );
}

#[test]
fn zip_int_int() {
    let ev = eval_program(
        r#"
get arr_zip from std::array
dec arr[int] a = [10, 20, 30]
dec arr[int] b = [1, 2, 3]
dec arr[(int, int)] z = arr_zip(a, b)
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("z"),
        Some(Value::Values {
            items_type: TypeAnnotation::Tuple(Rc::new(vec![
                TypeAnnotation::Null,
                TypeAnnotation::Null,
            ])),
            items: vec![
                Value::Tuple(vec![Value::Integer(10), Value::Integer(1)]),
                Value::Tuple(vec![Value::Integer(20), Value::Integer(2)]),
                Value::Tuple(vec![Value::Integer(30), Value::Integer(3)]),
            ],
        })
    );
}

#[test]
fn zip_truncates_to_shorter_left() {
    let ev = eval_program(
        r#"
get arr_zip, len from std::array
dec arr[int] a = [1, 2]
dec arr[int] b = [10, 20, 30, 40]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(2)));
}

#[test]
fn zip_truncates_to_shorter_right() {
    let ev = eval_program(
        r#"
get arr_zip, len from std::array
dec arr[int] a = [1, 2, 3, 4]
dec arr[int] b = [10, 20]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(2)));
}

#[test]
fn zip_empty_left() {
    let ev = eval_program(
        r#"
get arr_zip, len from std::array
dec arr[int] a = []
dec arr[int] b = [1, 2, 3]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(0)));
}

#[test]
fn zip_empty_right() {
    let ev = eval_program(
        r#"
get arr_zip, len from std::array
dec arr[int] a = [1, 2, 3]
dec arr[int] b = []
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(0)));
}

#[test]
fn zip_both_empty() {
    let ev = eval_program(
        r#"
get arr_zip, len from std::array
dec arr[int] a = []
dec arr[int] b = []
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(0)));
}

#[test]
fn zip_single_element() {
    let ev = eval_program(
        r#"
get arr_zip from std::array
dec arr[int] a = [42]
dec arr[string] b = ["hi"]
dec arr[(int, string)] z = arr_zip(a, b)
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("z"),
        Some(Value::Values {
            items_type: TypeAnnotation::Tuple(Rc::new(vec![
                TypeAnnotation::Null,
                TypeAnnotation::Null,
            ])),
            items: vec![Value::Tuple(vec![
                Value::Integer(42),
                Value::String("hi".to_string()),
            ])],
        })
    );
}

#[test]
fn zip_then_map() {
    let ev = eval_program(
        r#"
get arr_zip, len from std::array
dec arr[int] a = [1, 2, 3]
dec arr[int] b = [4, 5, 6]
dec arr[(int, int)] z = arr_zip(a, b)
dec int n = len(z)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("n"), Some(Value::Integer(3)));
}

#[test]
fn zip_non_array_first_arg_errors() {
    let result = eval_program(
        r#"
get arr_zip from std::array
dec arr[(int, int)] z = arr_zip(42, [1, 2])
"#,
    );
    assert!(result.is_err());
}

#[test]
fn zip_non_array_second_arg_errors() {
    let result = eval_program(
        r#"
get arr_zip from std::array
dec arr[(int, int)] z = arr_zip([1, 2], "oops")
"#,
    );
    assert!(result.is_err());
}
