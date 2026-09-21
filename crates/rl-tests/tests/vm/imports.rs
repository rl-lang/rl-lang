use crate::common;
use rl_vm::VmValue;

#[test]
fn wildcard_import_works() {
    let result = common::compile_and_run(
        r#"
        get * from std::math
        dec int x = abs(-5)?
        x
        "#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Int(5));
}

#[test]
fn aliased_import_works() {
    let result = common::compile_and_run(
        r#"
        get abs as absolute from std::math
        dec int x = absolute(-10)?
        x
        "#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Int(10));
}

#[test]
fn mixed_alias_and_plain_import_works() {
    let result = common::compile_and_run(
        r#"
        get abs as absolute, max from std::math
        dec int a = absolute(-3)?
        dec int b = max(10, 20)?
        (a, b)
        "#,
    )
    .expect("vm run failed");
    assert_eq!(
        result,
        VmValue::Tuple(std::rc::Rc::new(vec![VmValue::Int(3), VmValue::Int(20)]))
    );
}
