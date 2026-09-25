#![cfg(feature = "cc")]

use rl_checker::TypeChecker;
use rl_resolver::Resolver;
use rl_utils::source::SourceFile;

fn run_demo() -> String {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/scripts/transpile_demo.rl");
    let source = std::fs::read_to_string(&workspace)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", workspace.display(), e));

    let file = SourceFile::new("transpile_demo.rl", source);
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) =
        rl_parser::parser_logic::Parser::parse(tokens, file.clone()).expect("parse failed");

    let mut resolver = Resolver::new();
    let resolved = resolver.resolve_program(ast, stmts);

    let checker_tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (checker_ast, checker_stmts) =
        rl_parser::parser_logic::Parser::parse(checker_tokens, file).expect("parse failed");
    let mut checker = TypeChecker::new().with_ast_arena(checker_ast);
    let errors = checker.check(&checker_stmts);
    assert!(errors.is_empty(), "type check errors: {:?}", errors);

    let tmp = tempfile::tempdir().unwrap();
    let config = rl_cc::TranspileConfig {
        embed_runtime: true,
        output_dir: tmp.path().to_path_buf(),
        output_name: "demo_test".to_string(),
        test_mode: false,
        match_pattern: None,
    };
    let result =
        rl_cc::transpile(&resolver.ast_arena, &resolved, &checker, &config).expect("transpile failed");

    let cc_output = std::process::Command::new("cc")
        .args([
            "-std=c99",
            "-o",
            tmp.path().join("demo_test").to_str().unwrap(),
            result.c_path.to_str().unwrap(),
            tmp.path().join("rl_runtime.c").to_str().unwrap(),
            "-I",
            tmp.path().to_str().unwrap(),
            "-lm",
        ])
        .output()
        .expect("failed to run cc");
    assert!(
        cc_output.status.success(),
        "cc failed:\n{}",
        String::from_utf8_lossy(&cc_output.stderr)
    );

    let run_output = std::process::Command::new(tmp.path().join("demo_test"))
        .output()
        .expect("failed to run demo_test");
    assert!(
        run_output.status.success(),
        "demo_test failed:\n{}",
        String::from_utf8_lossy(&run_output.stderr)
    );

    String::from_utf8(run_output.stdout).unwrap()
}

#[test]
fn transpile_demo_compiles_and_runs() {
    let out = run_demo();

    assert!(out.contains("=== Arithmetic ===\n13\n7\n30\n3\n-10"));
    assert!(out.contains("=== Booleans ===\ntrue\nfalse"));
    assert!(out.contains("=== Floats ===\n3.14159"));
    assert!(out.contains("=== Strings ===\nHello, world!"));
    assert!(out.contains("=== Constants ===\n100"));
    assert!(out.contains("=== Null ===\nnull"));
    assert!(out.contains("=== Conditionals ===\nx is big"));
    assert!(out.contains("=== While Loop ===\n0\n1\n2"));
    assert!(out.contains("=== For Loop ===\n0\n1\n3"));
    assert!(out.contains("=== ForEach ===\n10\n20\n30"));
    assert!(out.contains("=== ForRange ===\n0\n1\n2\n3\n4"));
    assert!(out.contains("=== Loop ===\n0\n1\n2"));
    assert!(out.contains("=== Functions ===\n123\nfrom a function"));
    assert!(out.contains("=== Casts ===\n42"));
    assert!(out.contains("=== Tuples ===\n(1, 2, three)"));
    assert!(out.contains("=== Tuple Destruction ===\n42\nhello"));
    assert!(out.contains("=== Arrays ===\n10\n99"));
    assert!(out.contains("=== Records ===\n10\n30"));
    assert!(out.contains("=== Impl Methods ===\n50"));
    assert!(out.contains("=== Enums ===\nColor.Red"));
    assert!(out.contains("=== Match ===\nred"));
    assert!(out.contains("=== Results ===\nok(42)\nerr(1)\nok(5)"));
    assert!(out.contains("=== Maps ==="));
    assert!(out.contains("alice"));
    assert!(out.contains("=== Sets ==="));
    assert!(out.contains("done"));
}

#[test]
fn transpile_test_mode_compiles_and_runs() {
    let source = r#"
get println from std::io
get test_assert_eq, test_skip, test_run_registered from std::test

!#[test]
fn addition() {
    test_assert_eq(1 + 1, 2, "math")
}

!#[test]
fn skipped_case() {
    test_skip("later")
}

!#[test]
fn failing() {
    test_assert_eq(1, 2, "boom")
}

!#[test(register("inner"))]
fn inner_case() {
    test_assert_eq(1, 1, "in")
}

!#[test]
fn outer() {
    dec int f = test_run_registered("inner")
    test_assert_eq(f, 0, "lookup")
}

!#[test(cases(20))]
fn prop_squares(int x) {
    test_assert_eq(x * x >= 0, true, "sq")
}
"#;
    let file = SourceFile::new("cc_test_mode.rl", source.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) =
        rl_parser::parser_logic::Parser::parse(tokens, file.clone()).expect("parse failed");

    let mut resolver = Resolver::new();
    let resolved = resolver.resolve_program(ast, stmts);

    let checker_tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (checker_ast, checker_stmts) =
        rl_parser::parser_logic::Parser::parse(checker_tokens, file).expect("parse failed");
    let mut checker = TypeChecker::new().with_ast_arena(checker_ast);
    let errors = checker.check(&checker_stmts);
    assert!(errors.is_empty(), "type check errors: {:?}", errors);

    let tmp = tempfile::tempdir().unwrap();
    let config = rl_cc::TranspileConfig {
        embed_runtime: true,
        output_dir: tmp.path().to_path_buf(),
        output_name: "cc_test_mode".to_string(),
        test_mode: true,
        match_pattern: None,
    };
    let result =
        rl_cc::transpile(&resolver.ast_arena, &resolved, &checker, &config).expect("transpile failed");

    let cc_output = std::process::Command::new("cc")
        .args([
            "-std=c99",
            "-o",
            tmp.path().join("cc_test_mode").to_str().unwrap(),
            result.c_path.to_str().unwrap(),
            tmp.path().join("rl_runtime.c").to_str().unwrap(),
            "-I",
            tmp.path().to_str().unwrap(),
            "-lm",
        ])
        .output()
        .expect("failed to run cc");
    assert!(
        cc_output.status.success(),
        "cc failed:\n{}",
        String::from_utf8_lossy(&cc_output.stderr)
    );

    // One failure: non-zero exit, verdicts plus summary on stdout.
    let run_output = std::process::Command::new(tmp.path().join("cc_test_mode"))
        .output()
        .expect("failed to run cc_test_mode");
    assert!(!run_output.status.success());
    let out = String::from_utf8(run_output.stdout).unwrap();
    assert!(out.contains("ok addition"));
    assert!(out.contains("SKIP [skipped_case] skipped: later"));
    assert!(out.contains("FAIL failing"));
    assert!(out.contains("ok inner_case"));
    assert!(out.contains("ok outer"));
    assert!(out.contains("ok prop_squares"));
    assert!(out.contains("ran 6 tests: 4 ok, 1 failed, 1 skipped"));
}

#[test]
fn transpile_buffers_match_vm() {
    let source = r#"
get __buf_new, __buf_push_byte, __buf_append from core
get __buf_get_byte, __buf_set_byte, __buf_slice from core
get __buf_to_string, __buf_len, __buf_clear from core
get __buf_resize, __buf_addr, __buf_free from core
get to_int from std::types
get result_unwrap from std::res
get println from std::io

dec b = __buf_new()
__buf_push_byte(b, 72 as byte)
__buf_push_byte(b, 105 as byte)
__buf_append(b, "!")
println(__buf_len(b))
println(result_unwrap(to_int(__buf_get_byte(b, 0))))
__buf_set_byte(b, 0, 104 as byte)
println(__buf_slice(b, 0, 3))
println(__buf_to_string(b))
__buf_clear(b)
println(__buf_len(b))
__buf_resize(b, 4)
println(__buf_len(b))
println(__buf_addr(b) != 0)
__buf_free(b)
"#;
    let file = SourceFile::new("cc_buffers.rl", source.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) =
        rl_parser::parser_logic::Parser::parse(tokens, file.clone()).expect("parse failed");

    let mut resolver = Resolver::new();
    let resolved = resolver.resolve_program(ast, stmts);

    let checker_tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (checker_ast, checker_stmts) =
        rl_parser::parser_logic::Parser::parse(checker_tokens, file).expect("parse failed");
    let mut checker = TypeChecker::new().with_ast_arena(checker_ast);
    let errors = checker.check(&checker_stmts);
    assert!(errors.is_empty(), "type check errors: {:?}", errors);

    let tmp = tempfile::tempdir().unwrap();
    let config = rl_cc::TranspileConfig {
        embed_runtime: true,
        output_dir: tmp.path().to_path_buf(),
        output_name: "cc_buffers".to_string(),
        test_mode: false,
        match_pattern: None,
    };
    let result =
        rl_cc::transpile(&resolver.ast_arena, &resolved, &checker, &config).expect("transpile failed");

    let cc_output = std::process::Command::new("cc")
        .args([
            "-std=c99",
            "-o",
            tmp.path().join("cc_buffers").to_str().unwrap(),
            result.c_path.to_str().unwrap(),
            tmp.path().join("rl_runtime.c").to_str().unwrap(),
            "-I",
            tmp.path().to_str().unwrap(),
            "-lm",
        ])
        .output()
        .expect("failed to run cc");
    assert!(
        cc_output.status.success(),
        "cc failed:\n{}",
        String::from_utf8_lossy(&cc_output.stderr)
    );

    let run_output = std::process::Command::new(tmp.path().join("cc_buffers"))
        .output()
        .expect("failed to run cc_buffers");
    assert!(run_output.status.success());
    let out = String::from_utf8(run_output.stdout).unwrap();
    assert_eq!(out, "3\n72\nhi!\nhi!\n0\n4\ntrue\n");
}
