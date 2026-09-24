use rl_vm::VmValue;

use crate::common::compile_and_run;

// NOTE: parse_args reads the real process argv, so tests only cover the
// argv-independent paths (empty spec, malformed spec). Flag parsing itself is
// exercised by rl-examples/scripts/cli_demo.rl with controlled arguments.

#[test]
fn shell_split_basic() {
    let result = compile_and_run(
        r#"
get shell_split from std::cli
get result_unwrap from std::res
get len from std
dec parts = result_unwrap(shell_split("a b \"c d\""))
result_unwrap(len(parts))
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn shell_split_unterminated_is_err() {
    let result = compile_and_run(
        r#"
get shell_split from std::cli
get is_err from std::res
dec bool x = is_err(shell_split("\"abc"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn shell_join_round_trips() {
    let result = compile_and_run(
        r#"
get shell_split, shell_join from std::cli
get result_unwrap from std::res
get len from std
dec parts = result_unwrap(shell_split(shell_join(["a", "b c"])))
result_unwrap(len(parts))
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn usage_string_mentions_options() {
    let result = compile_and_run(
        r#"
get usage_string from std::cli
get result_unwrap from std::res
get contains from std::str
dec text = result_unwrap(usage_string([{"name": "verbose", "short": "v", "flag": "true"}]))
contains(text, "--verbose")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn usage_string_bad_spec_is_err() {
    let result = compile_and_run(
        r#"
get usage_string from std::cli
get is_err from std::res
dec bool x = is_err(usage_string([{"nope": "x"}]))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn parse_args_empty_spec_is_ok() {
    let result = compile_and_run(
        r#"
get parse_args from std::cli
get is_ok from std::res
dec bool x = is_ok(parse_args([]))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn parse_args_missing_name_is_err() {
    let result = compile_and_run(
        r#"
get parse_args from std::cli
get is_err from std::res
dec bool x = is_err(parse_args([{"nope": "x"}]))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn parse_args_bad_flag_word_is_err() {
    let result = compile_and_run(
        r#"
get parse_args from std::cli
get is_err from std::res
dec bool x = is_err(parse_args([{"name": "v", "flag": "maybe"}]))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn parse_args_flag_defaults_false() {
    let result = compile_and_run(
        r#"
get parse_args from std::cli
get result_unwrap from std::res
get map_get from std::collections
dec args = result_unwrap(parse_args([{"name": "verbose", "flag": "true"}]))
dec bool x = map_get(args, "verbose").result_unwrap()
x
"#,
    )
    .unwrap();
    // NOTE: passes only when the test runner argv carries no --verbose,
    // which holds for plain `cargo test` invocations.
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn progress_bar_returns_null() {
    let result = compile_and_run(
        r#"
get progress_bar from std::cli
progress_bar(5, 10, "t")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Null);
}

#[test]
fn spinner_tick_returns_null() {
    let result = compile_and_run(
        r#"
get spinner_tick from std::cli
spinner_tick(3)
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Null);
}
