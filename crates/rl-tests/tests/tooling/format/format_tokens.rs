use rl_tooling::format::format_tokens;

use crate::common;

#[test]
fn format_simple_expression() {
    let tokens = common::lex(" dec  int x =  1000");
    let out = format_tokens(&tokens);
    assert_eq!(out, "dec int x = 1000");
}

#[test]
fn format_function_call() {
    let tokens = common::lex("println    (\"hello\")");
    let out = format_tokens(&tokens);
    assert_eq!(out, "println(\"hello\")");
}

#[test]
fn format_indentation_with_braces() {
    let tokens = common::lex(
        r#"fn main(){
println("hello")
}"#,
    );

    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        r#"fn main()
{
    println("hello")
}"#
    )
}

#[test]
fn format_nested_blocks() {
    let tokens = common::lex(
        r#"if x {
if y {
hello()
}
}"#,
    );

    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        r#"if x
{
    if y
    {
        hello()
    }
}"#
    );
}

#[test]
fn format_line_comments() {
    let tokens = common::lex(
        r#"//hello
dec int x = 1"#,
    );

    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        r#"// hello
dec int x = 1"#
    );
}

#[test]
fn format_doc_comments() {
    let tokens = common::lex(
        r#"/// docs
fn main()"#,
    );

    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        r#"/// docs
fn main()"#
    );
}

#[test]
fn format_blank_lines() {
    let tokens = common::lex(
        r#"
    
    
    
dec int x = 1"#,
    );

    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        r#"

dec int x = 1"#
    );
}

#[test]
fn format_unary_minus() {
    let tokens = common::lex("x = - 10");

    let out = format_tokens(&tokens);

    assert_eq!(out, "x = -10");
}

#[test]
fn format_binary_minus() {
    let tokens = common::lex("x = 10-5");

    let out = format_tokens(&tokens);

    assert_eq!(out, "x = 10 - 5");
}

#[test]
fn format_array_indexing() {
    let tokens = common::lex("arr [0]");

    let out = format_tokens(&tokens);

    assert_eq!(out, "arr[0]");
}

#[test]
fn format_generic_type_brackets() {
    let tokens = common::lex("arr [int]");

    let out = format_tokens(&tokens);

    assert_eq!(out, "arr[int]");
}

#[test]
fn format_function_call_with_arguments() {
    let tokens = common::lex("foo(1,2)");

    let out = format_tokens(&tokens);

    assert_eq!(out, "foo(1, 2)");
}

#[test]
fn format_else_on_own_line() {
    let tokens = common::lex("if x {\nprintln(x)\n} else {\nprintln(0)\n}");
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        "if x\n{\n    println(x)\n}\nelse\n{\n    println(0)\n}"
    );
}

#[test]
fn format_single_line_block_preserved() {
    let tokens = common::lex("match s {\n\"a\" => { x = 1 }\n_ => {\ny = 2\n}\n}");
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        "match s\n{\n    \"a\" => { x = 1 }\n    _ =>\n    {\n        y = 2\n    }\n}"
    );
}

#[test]
fn format_short_fn_signature_stays_inline() {
    let tokens = common::lex("fn add(int a, int b) -> int {\nreturn a + b\n}");
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        "fn add(int a, int b) -> int\n{\n    return a + b\n}"
    );
}

#[test]
fn format_long_fn_signature_wraps() {
    let tokens = common::lex(
        "fn do_more(int alpha, int beta, string gamma, bool delta) -> int {\nreturn 1\n}",
    );
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        "fn do_more(\n    int alpha, int beta, string gamma, bool delta\n    ) -> int\n{\n    return 1\n}"
    );
}

#[test]
fn format_record_fields_one_per_line() {
    let tokens = common::lex("record P { int x, int y }");
    let out = format_tokens(&tokens);
    assert_eq!(out, "record P\n{\n    int x,\n    int y,\n}");
}

#[test]
fn format_leading_comment_no_extra_blank() {
    let tokens = common::lex("// top comment\ndec int x = 1\n");
    let out = format_tokens(&tokens);
    assert_eq!(out, "// top comment\ndec int x = 1\n");
}

#[test]
fn format_wide_array_breaks_vertical() {
    let tokens = common::lex("dec arr[string] r = [\"31\", \"32\", \"33\", \"34\", \"35\", \"36\"]");
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        "dec arr[string] r = [\n    \"31\",\n    \"32\",\n    \"33\",\n    \"34\",\n    \"35\",\n    \"36\",\n]"
    );
}

#[test]
fn format_narrow_array_stays_inline() {
    let tokens = common::lex("dec arr[int] r = [1, 2]");
    let out = format_tokens(&tokens);
    assert_eq!(out, "dec arr[int] r = [1, 2]");
}

#[test]
fn format_wide_call_breaks_vertical() {
    let tokens = common::lex("dec string o = some_quite_long_function_name(\"first longish argument here\", \"second\")");
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        "dec string o = some_quite_long_function_name(\n    \"first longish argument here\",\n    \"second\"\n)"
    );
}

#[test]
fn format_nested_calls_break_recursively() {
    let tokens = common::lex("outer_function_name_here(format_the_thing_with_long_name(\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", \"b\"), \"second argument here\")");
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        "outer_function_name_here(\n    format_the_thing_with_long_name(\n        \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\n        \"b\"\n    ),\n    \"second argument here\"\n)"
    );
}

#[test]
fn format_is_idempotent() {
    use rl_tooling::format::format_tokens_with;
    use rl_tooling::format::FormatOptions;
    let source = r#"get println from std::io
record P { int x, int y }
fn add(int a, int b) -> int {
return a + b
}
fn do_more(int alpha, int beta, string gamma, bool delta) -> int {
return 1
}
match s {
"a" => { x = 1 }
_ => {
y = 2
}
}
if x {
println(x)
} else {
println(0)
}
dec int q = 1;dec int w = 2
dec arr[string] r = ["31", "32", "33", "34", "35", "36"]
dec string o = some_quite_long_function_name("first longish argument here", "second")"#;
    let opts = FormatOptions::default();
    let once = format_tokens_with(&common::lex(source), &opts);
    let twice = format_tokens_with(&common::lex(&once), &opts);
    assert_eq!(once, twice, "formatter output changed on second pass:\n{once}");
}
