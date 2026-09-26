use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

fn temp_path(name: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("rl-vm-fs-{}-{}", std::process::id(), name));
    path
}

#[test]
fn fs_read_file_works() {
    let path = temp_path("fs_read.txt");
    std::fs::write(&path, "hello from fs").unwrap();
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get read_file from std::fs
get result_unwrap from std::res
dec result[string] r = read_file("{path_str}")
dec string x = result_unwrap(r)
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("hello from fs")));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn fs_read_lines_works() {
    let path = temp_path("fs_lines.txt");
    std::fs::write(&path, "one\ntwo\nthree").unwrap();
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get read_lines from std::fs
get result_unwrap from std::res
dec result[arr[string]] r = read_lines("{path_str}")
dec arr[string] lines = result_unwrap(r)
lines
"#
    ))
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![
            VmValue::Str(Rc::from("one")),
            VmValue::Str(Rc::from("two")),
            VmValue::Str(Rc::from("three")),
        ]))
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn fs_write_file_works() {
    let path = temp_path("fs_write.txt");
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get write_file from std::fs
get is_ok from std::res
dec result[string] r = write_file("{path_str}", "written by rl")
dec bool x = is_ok(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "written by rl");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn fs_append_file_works() {
    let path = temp_path("fs_append.txt");
    std::fs::write(&path, "first").unwrap();
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get append_file from std::fs
get is_ok from std::res
dec result[string] r = append_file("{path_str}", "-second")
dec bool x = is_ok(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "first-second");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn fs_delete_file_works() {
    let path = temp_path("fs_delete.txt");
    std::fs::write(&path, "bye").unwrap();
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get delete_file from std::fs
get is_ok from std::res
dec result[string] r = delete_file("{path_str}")
dec bool x = is_ok(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    assert!(!path.exists());
}

#[test]
fn fs_path_exists_true() {
    let result = compile_and_run(
        r#"
get path_exists from std::fs
path_exists("/tmp")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn fs_path_exists_false() {
    let result = compile_and_run(
        r#"
get path_exists from std::fs
path_exists("/tmp/this-does-not-exist-rl-test")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn fs_path_is_dir_true() {
    let result = compile_and_run(
        r#"
get path_is_dir from std::fs
path_is_dir("/tmp")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn fs_path_is_file_false_for_dir() {
    let result = compile_and_run(
        r#"
get path_is_file from std::fs
path_is_file("/tmp")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn fs_read_file_resolves_without_warning() {
    let checker = crate::common::check(
        r#"
fn test_fn() -> string {
    dec result[string] r = std::fs::read_file("x.txt")
    dec string x = std::res::result_unwrap(r)
    return x
}
"#,
    );
    let warnings: Vec<String> = checker.warnings.iter().map(|w| w.message().to_string()).collect();
    assert!(
        !warnings.iter().any(|m| m.contains("deprecated")),
        "expected no deprecation warnings, got: {:?}",
        warnings
    );
}

#[test]
fn fs_path_exists_resolves_without_warning() {
    let checker = crate::common::check(
        r#"
fn test_fn() -> bool {
    return std::fs::path_exists("/tmp")
}
"#,
    );
    let warnings: Vec<String> = checker.warnings.iter().map(|w| w.message().to_string()).collect();
    assert!(
        !warnings.iter().any(|m| m.contains("deprecated")),
        "expected no deprecation warnings, got: {:?}",
        warnings
    );
}
