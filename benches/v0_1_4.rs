use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rl_lang::interpreter::evaluator::Evaluator;
use rl_lang::lexer::tokenizer::Tokenizer;
use rl_lang::parser::parser_logic::Parser;
use rl_lang::utils::source::SourceFile;

// ===== source snippets =====

// --- declarations ---
const SRC_DEC_INT: &str = "dec int x = 0";
const SRC_CONST_INT: &str = "CONST int x = 0";
const SRC_DEC_FLOAT: &str = "dec float x = 0.0";
const SRC_CONST_FLOAT: &str = "CONST float x = 0.0";
const SRC_DEC_STRING: &str = r#"dec string x = "hi""#;
const SRC_CONST_STRING: &str = r#"CONST string x = "hi""#;
const SRC_DEC_BOOL: &str = "dec bool x = true";
const SRC_CONST_BOOL: &str = "CONST bool x = false";
const SRC_DEC_CHAR: &str = "dec char x = 'x'";
const SRC_CONST_CHAR: &str = "CONST char x = 'x'";
const SRC_DEC_ARRAY: &str = "dec arr[int] x = [1, 2, 3]";
const SRC_CONST_ARRAY: &str = "CONST arr[int] x = [1, 2, 3]";
const SRC_DEC_FN: &str = "dec fn x = fn() {}";
const SRC_CONST_FN: &str = "CONST fn x = fn() {}";
const SRC_DEC_TUPLE: &str = r#"dec (int, string) x = (42, "hello")"#;
const SRC_DEC_ERROR: &str = r#"dec error e = error("oops")"#;
const SRC_DEC_RESULT_OK: &str = "dec result[int] r = ok(42)";
const SRC_DEC_RESULT_ERR: &str = r#"dec result[int] r = err("oops")"#;

// --- control flow ---
const SRC_WHILE: &str = "while true { 0 }";
const SRC_IF_SIMPLE: &str = "if true { 0 }";
const SRC_IF_ELSE: &str = "if true { 1 } else { 0 }";
const SRC_IF_ELSE_IF: &str = "if true { 1 } else if false { 2 }";
const SRC_IF_ELSE_IF_ELSE: &str = "if true { 1 } else if false { 2 } else { 0 }";
const SRC_FOR_C: &str = "for [dec int i = 0, i < 10, i += 1] { 0 }";
const SRC_FOR_RANGE: &str = "for i in 0..10 { 0 }";
const SRC_FOR_EACH: &str = "for i in [1, 2, 3, 4, 5] { 0 }";

// --- functions ---
const SRC_FN_SIMPLE: &str = "fn add(int a, int b) -> int { return a + b }";
const SRC_FN_FN_PARAM: &str = "fn apply(fn f, int x) -> int { return f(x) }";
const SRC_FN_RECURSIVE: &str =
    "fn fact(int n) -> int { if n <= 1 { return 1 } return n * fact(n - 1) }";
const SRC_FN_RESULT_RETURN: &str = "fn safe_div(int a, int b) -> result[int] { if b == 0 { return err(\"div by zero\") } return ok(a / b) }";
const SRC_LAMBDA: &str = "dec fn f = fn(int n) -> int { return n * 2 }";

// --- imports ---
const SRC_IMPORT_SIMPLE: &str = "get println from std::io";
const SRC_IMPORT_MULTI: &str = "get abs, min, max from std::math";
const SRC_IMPORT_RESULT: &str = "get is_ok, is_err, result_unwrap from std::res";

// --- programs ---
const SRC_PROGRAM_RESULT_CHAIN: &str = "\
get is_ok, is_err, result_unwrap, result_unwrap_or, result_map from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err(\"division by zero\")
    }
    return ok(a / b)
}
dec result[int] r1 = safe_div(10, 2)
dec result[int] r2 = safe_div(10, 0)
dec result[int] r3 = result_map(r1, fn(int n) -> int { return n * 2 })
dec int v1 = result_unwrap(r1)
dec int v2 = result_unwrap_or(r2, -1)
dec bool b1 = is_ok(r3)
dec bool b2 = is_err(r2)";

const SRC_PROGRAM_ARR_ZIP: &str = "\
get arr_zip, arr_map, arr_filter, arr_reduce from std::array
dec arr[int] xs = [1, 2, 3, 4, 5]
dec arr[int] ys = [10, 20, 30, 40, 50]
dec arr[(int, int)] pairs = arr_zip(xs, ys)
dec arr[int] sums = arr_map(xs, fn(int n) -> int { return n * 2 })
dec arr[int] big = arr_filter(xs, fn(int n) -> bool { return n > 2 })
dec int total = arr_reduce(xs, fn(int acc, int n) -> int { return acc + n }, 0)";

const SRC_PROGRAM_FIBONACCI: &str = "\
fn fib(int n) -> int {
    if n <= 1 {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}
dec int a = fib(10)
dec int b = fib(15)
dec int c = fib(20)";

const SRC_PROGRAM_TUPLE_DESTRUCTURE: &str = r#"
dec (int, string, bool) t = (42, "hello", true)
dec int a = t[0]
dec string b = t[1]
dec bool c = t[2]
dec int x, string y, bool z = (1, "world", false)
"#;

const SRC_PROGRAM_CLOSURE: &str = "\
dec int base = 100
dec fn add_base = fn(int n) -> int { return n + base }
dec fn mul_base = fn(int n) -> int { return n * base }
dec int r1 = add_base(5)
dec int r2 = mul_base(3)
dec int r3 = add_base(mul_base(2))";

// ===== helpers =====

fn src(name: &str, text: &str) -> SourceFile {
    SourceFile::new(name, text.to_string())
}

fn lex_only(text: &str) {
    let _ = Tokenizer::lex(src("bench", black_box(text)));
}

fn lex_and_parse(text: &str) {
    let sf = src("bench", black_box(text));
    let tokens = match Tokenizer::lex(sf.clone()) {
        Ok(t) => t,
        Err(_) => return,
    };
    let _ = Parser::parse(tokens, sf);
}

fn full_pipeline(text: &str) {
    let sf = src("bench", black_box(text));
    let tokens = match Tokenizer::lex(sf.clone()) {
        Ok(t) => t,
        Err(_) => return,
    };
    let (ast, stmts) = match Parser::parse(tokens, sf.clone()) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut ev = Evaluator::default().with_stdlib().with_source_file(sf);
    let stmts = ev.resolver.resolve_program(ast, stmts);
    let _ = ev.evaluate_program(&stmts);
}

// ===== benchmark groups =====

fn bench_v0_1_4_lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("v0.1.4/lexer");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(200);

    // declarations
    for (name, src_str) in [
        ("dec_int", SRC_DEC_INT),
        ("const_int", SRC_CONST_INT),
        ("dec_float", SRC_DEC_FLOAT),
        ("const_float", SRC_CONST_FLOAT),
        ("dec_string", SRC_DEC_STRING),
        ("const_string", SRC_CONST_STRING),
        ("dec_bool", SRC_DEC_BOOL),
        ("const_bool", SRC_CONST_BOOL),
        ("dec_char", SRC_DEC_CHAR),
        ("const_char", SRC_CONST_CHAR),
        ("dec_array", SRC_DEC_ARRAY),
        ("const_array", SRC_CONST_ARRAY),
        ("dec_fn", SRC_DEC_FN),
        ("const_fn", SRC_CONST_FN),
        ("dec_tuple", SRC_DEC_TUPLE),
        ("dec_error", SRC_DEC_ERROR),
        ("dec_result_ok", SRC_DEC_RESULT_OK),
        ("dec_result_err", SRC_DEC_RESULT_ERR),
    ] {
        group.bench_with_input(BenchmarkId::new("declaration", name), src_str, |b, s| {
            b.iter(|| lex_only(s))
        });
    }

    // control flow
    for (name, src_str) in [
        ("while", SRC_WHILE),
        ("if_simple", SRC_IF_SIMPLE),
        ("if_else", SRC_IF_ELSE),
        ("if_else_if", SRC_IF_ELSE_IF),
        ("if_else_if_else", SRC_IF_ELSE_IF_ELSE),
        ("for_c", SRC_FOR_C),
        ("for_range", SRC_FOR_RANGE),
        ("for_each", SRC_FOR_EACH),
    ] {
        group.bench_with_input(BenchmarkId::new("control_flow", name), src_str, |b, s| {
            b.iter(|| lex_only(s))
        });
    }

    // functions
    for (name, src_str) in [
        ("fn_simple", SRC_FN_SIMPLE),
        ("fn_fn_param", SRC_FN_FN_PARAM),
        ("fn_recursive", SRC_FN_RECURSIVE),
        ("fn_result_return", SRC_FN_RESULT_RETURN),
        ("lambda", SRC_LAMBDA),
    ] {
        group.bench_with_input(BenchmarkId::new("function", name), src_str, |b, s| {
            b.iter(|| lex_only(s))
        });
    }

    // imports
    for (name, src_str) in [
        ("import_simple", SRC_IMPORT_SIMPLE),
        ("import_multi", SRC_IMPORT_MULTI),
        ("import_result", SRC_IMPORT_RESULT),
    ] {
        group.bench_with_input(BenchmarkId::new("import", name), src_str, |b, s| {
            b.iter(|| lex_only(s))
        });
    }

    // programs
    for (name, src_str) in [
        ("result_chain", SRC_PROGRAM_RESULT_CHAIN),
        ("arr_zip", SRC_PROGRAM_ARR_ZIP),
        ("fibonacci", SRC_PROGRAM_FIBONACCI),
        ("tuple_destructure", SRC_PROGRAM_TUPLE_DESTRUCTURE),
        ("closure", SRC_PROGRAM_CLOSURE),
    ] {
        group.bench_with_input(BenchmarkId::new("program", name), src_str, |b, s| {
            b.iter(|| lex_only(s))
        });
    }

    group.finish();
}

fn bench_v0_1_4_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("v0.1.4/parser");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(200);

    // declarations
    for (name, src_str) in [
        ("dec_int", SRC_DEC_INT),
        ("const_int", SRC_CONST_INT),
        ("dec_float", SRC_DEC_FLOAT),
        ("const_float", SRC_CONST_FLOAT),
        ("dec_string", SRC_DEC_STRING),
        ("const_string", SRC_CONST_STRING),
        ("dec_bool", SRC_DEC_BOOL),
        ("const_bool", SRC_CONST_BOOL),
        ("dec_char", SRC_DEC_CHAR),
        ("const_char", SRC_CONST_CHAR),
        ("dec_array", SRC_DEC_ARRAY),
        ("const_array", SRC_CONST_ARRAY),
        ("dec_fn", SRC_DEC_FN),
        ("const_fn", SRC_CONST_FN),
        ("dec_tuple", SRC_DEC_TUPLE),
        ("dec_error", SRC_DEC_ERROR),
        ("dec_result_ok", SRC_DEC_RESULT_OK),
        ("dec_result_err", SRC_DEC_RESULT_ERR),
    ] {
        group.bench_with_input(BenchmarkId::new("declaration", name), src_str, |b, s| {
            b.iter(|| lex_and_parse(s))
        });
    }

    // control flow
    for (name, src_str) in [
        ("while", SRC_WHILE),
        ("if_simple", SRC_IF_SIMPLE),
        ("if_else", SRC_IF_ELSE),
        ("if_else_if", SRC_IF_ELSE_IF),
        ("if_else_if_else", SRC_IF_ELSE_IF_ELSE),
        ("for_c", SRC_FOR_C),
        ("for_range", SRC_FOR_RANGE),
        ("for_each", SRC_FOR_EACH),
    ] {
        group.bench_with_input(BenchmarkId::new("control_flow", name), src_str, |b, s| {
            b.iter(|| lex_and_parse(s))
        });
    }

    // functions
    for (name, src_str) in [
        ("fn_simple", SRC_FN_SIMPLE),
        ("fn_fn_param", SRC_FN_FN_PARAM),
        ("fn_recursive", SRC_FN_RECURSIVE),
        ("fn_result_return", SRC_FN_RESULT_RETURN),
        ("lambda", SRC_LAMBDA),
    ] {
        group.bench_with_input(BenchmarkId::new("function", name), src_str, |b, s| {
            b.iter(|| lex_and_parse(s))
        });
    }

    // imports
    for (name, src_str) in [
        ("import_simple", SRC_IMPORT_SIMPLE),
        ("import_multi", SRC_IMPORT_MULTI),
        ("import_result", SRC_IMPORT_RESULT),
    ] {
        group.bench_with_input(BenchmarkId::new("import", name), src_str, |b, s| {
            b.iter(|| lex_and_parse(s))
        });
    }

    // programs at repeat counts to observe scaling
    for (name, src_str) in [
        ("result_chain", SRC_PROGRAM_RESULT_CHAIN),
        ("arr_zip", SRC_PROGRAM_ARR_ZIP),
        ("fibonacci", SRC_PROGRAM_FIBONACCI),
        ("tuple_destructure", SRC_PROGRAM_TUPLE_DESTRUCTURE),
        ("closure", SRC_PROGRAM_CLOSURE),
    ] {
        for size in [1, 10, 50] {
            let text = src_str.repeat(size);
            group.bench_with_input(
                BenchmarkId::new(format!("program/{}", name), size),
                &text,
                |b, s| b.iter(|| lex_and_parse(black_box(s))),
            );
        }
    }

    group.finish();
}

fn bench_v0_1_4_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("v0.1.4/pipeline");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(100);

    for (name, src_str) in [
        ("result_chain", SRC_PROGRAM_RESULT_CHAIN),
        ("arr_zip", SRC_PROGRAM_ARR_ZIP),
        ("fibonacci", SRC_PROGRAM_FIBONACCI),
        ("tuple_destructure", SRC_PROGRAM_TUPLE_DESTRUCTURE),
        ("closure", SRC_PROGRAM_CLOSURE),
    ] {
        group.bench_with_input(BenchmarkId::new("program", name), src_str, |b, s| {
            b.iter(|| full_pipeline(black_box(s)))
        });
    }

    group.finish();
}

criterion_group!(
    benches_v0_1_4,
    bench_v0_1_4_lexer,
    bench_v0_1_4_parser,
    bench_v0_1_4_pipeline
);
criterion_main!(benches_v0_1_4);
