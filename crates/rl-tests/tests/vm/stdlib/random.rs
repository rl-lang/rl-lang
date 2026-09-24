use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn rand_seed_deterministic() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_int from std::random

rand_seed(42)
dec int a = rand_int()
rand_seed(42)
dec int b = rand_int()
a
"#,
    )
    .unwrap();
    let result2 = compile_and_run(
        r#"
get rand_seed from std::random
get rand_int from std::random

rand_seed(42)
dec int a = rand_int()
rand_seed(42)
dec int b = rand_int()
b
"#,
    )
    .unwrap();
    assert_eq!(result, result2);
}

#[test]
fn rand_seed_changes_output() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_int from std::random

rand_seed(1)
dec int a = rand_int()
rand_seed(2)
dec int b = rand_int()
a != b
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn rand_int_range() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_int_range from std::random

rand_seed(42)
dec int v = rand_int_range(1, 10)?
v >= 1 and v <= 10
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn rand_float_range() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_float_range from std::random

rand_seed(42)
dec float v = rand_float_range(0.0, 1.0)?
v >= 0.0 and v < 1.0
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn rand_bool_weighted() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_bool_weighted from std::random

rand_seed(42)
dec bool v = rand_bool_weighted(1.0)
v
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn rand_dice() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_dice from std::random

rand_seed(42)
dec int v = rand_dice(6)?
v >= 1 and v <= 6
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn rand_choice() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_choice from std::random

rand_seed(42)
dec arr[int] items = [10, 20, 30]
dec int v = rand_choice(items)?
v == 10 or v == 20 or v == 30
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn rand_shuffle() {
    let result = compile_and_run(
        r#"
get rand_seed from std::random
get rand_shuffle from std::random
get arr_count from std::array

rand_seed(42)
dec arr[int] items = [1, 2, 3, 4, 5]
dec arr[int] shuffled = rand_shuffle(items)?
dec int n = arr_count(shuffled)?
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(5));
}
