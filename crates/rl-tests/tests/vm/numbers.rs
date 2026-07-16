use crate::common;
use rl_vm::VmValue;

#[test]
fn basic_variable_arithemtic() {
    let result = common::compile_and_run(
        r#"
        1 + 2 * 4 * 4 + 342 * 1
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(375))
}
