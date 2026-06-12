use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rl_lang::lexer::tokenizer::Tokenizer;
use rl_lang::parser::parser_logic::Parser;
use rl_lang::utils::source::SourceFile;

// ===== source snippets =====

const SRC_FOR_C: &str = "for [int i = 1, i < 10, i += 1] {0}";

const SRC_FOR_RANGE: &str = "for i in 1..10 {0}";

const SRC_FOR_ITERABLE: &str = "for i in [1,2,3,4,5,6,7,8,9] {0}";

const SRC_IMPORT_SIMPLE: &str = "get x from y";
const SRC_IMPORT_PATH: &str = "get x from y::z";
const SRC_IMPORT_MULTI: &str = "get x, z from y";
const SRC_IMPORT_MULTI_PATH: &str = "get x, z from y::w";
const SRC_IMPORT_FILE: &str = "get x";
const SRC_IMPORT_FILE_PATH: &str = "get x::y";

const SRC_DEC_INT: &str = "dec int x = 0";
const SRC_CONST_INT: &str = "CONST int x = 0";
const SRC_DEC_FLOAT: &str = "dec float x = 0.0";
const SRC_CONST_FLOAT: &str = "CONST float x = 0.0";
const SRC_DEC_STRING: &str = "dec string x = \"hi\"";
const SRC_CONST_STRING: &str = "CONST string x = \"hi\"";
const SRC_DEC_CHAR: &str = "dec char x = 'x'";
const SRC_CONST_CHAR: &str = "CONST char x = 'x'";
const SRC_DEC_BOOL: &str = "dec bool x = true";
const SRC_CONST_BOOL: &str = "CONST bool x = false";
const SRC_DEC_ARRAY: &str = "dec arr[int] x = [1]";
const SRC_CONST_ARRAY: &str = "CONST arr[int] x = [1]";
const SRC_DEC_FN: &str = "dec fn x = fn(){}";
const SRC_CONST_FN: &str = "CONST fn x = fn(){}";

const SRC_WHILE: &str = "while (true) {0}";

const SRC_IF_SIMPLE: &str = "if (true) {0}";
const SRC_IF_ELSE: &str = "if (true) {1} else {0}";
const SRC_IF_ELSE_IF: &str = "if (true) {1} else if (false) {2}";
const SRC_IF_ELSE_IF_ELSE: &str = "if (true) {1} else if (false) {2} else {0}";
const SRC_IF_NESTED: &str = "if (true) { if (false) {0} else {1} } else {0}";

const SRC_FN_SIMPLE: &str = "fn x (int x) {return x}";
const SRC_FN_FN_PARAM: &str = "fn x (fn x, int y) {return x(y)}";
const SRC_DEC_FN_LAMBDA: &str = "dec fn x = fn(int x) {return x}";

const SRC_PROGRAM_MATH_PRINT: &str = "\
get println from std::display
get pow from std::math
get std::math::consts::PI

CONST float pi = PI()
dec float x = pow(pi, pi)

println(x, y)";

const SRC_PROGRAM_GEOMETRY: &str = "\
get println from std::display
get sin, cos, hypot, is_prime, log2, factorial from std::math
get PI, TAU, PHI from std::math::consts

dec float angle = PI()
angle *= PHI()

dec float opp = sin(angle)
dec float adj = cos(angle)
dec float hyp = hypot(opp, adj)

dec bool is_right = hyp == 1.0
dec bool not_right = !is_right

dec int n = 7
dec bool prime = is_prime(n)
dec float fact = factorial(n)
dec float lg = log2(fact)

lg -= 1.0

dec bool big = lg > TAU()

println(hyp, is_right, prime, big)";

const SRC_PROGRAM_FN_ARRAY: &str = "\
get println from std::display
get is_prime, factorial, fibonacci from std::math

dec int n = 10
dec arr[fn] ops = [factorial, fibonacci]

dec int i = 0
for [int i = 0, i < 2, i += 1] {
    dec fn op = ops[i]
    dec int j = 1
    while (j < n) {
        dec int result = op(j)
        if (is_prime(result)) {
            println(result)
            j += 1
        } else {
            break
        }
    }
    i += 1
}";

// ===== helpers =====

fn src(name: &str, text: &str) -> SourceFile {
    SourceFile::new(name, text.to_string())
}

fn lex_and_parse(text: &str) {
    let sf = src("bench", black_box(text));
    let tokens = Tokenizer::lex(sf.clone());
    let _ = match tokens {
        Ok(tokens) => Parser::parse(tokens, sf),
        Err(_) => {
            eprintln!("Error had occured");
            return;
        }
    };
}

// ===== benchmark groups =====

fn bench_v0_1_0_lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("v0.1.0/lexer");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(200);

    // for loops
    group.bench_function("for_c", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_FOR_C))))
    });
    group.bench_function("for_range", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_FOR_RANGE))))
    });
    group.bench_function("for_iterable", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_FOR_ITERABLE))))
    });

    // imports
    group.bench_function("import_simple", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_IMPORT_SIMPLE))))
    });
    group.bench_function("import_path", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_IMPORT_PATH))))
    });
    group.bench_function("import_multi", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_IMPORT_MULTI))))
    });
    group.bench_function("import_multi_path", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_IMPORT_MULTI_PATH))))
    });
    group.bench_function("import_file", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_IMPORT_FILE))))
    });
    group.bench_function("import_file_path", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_IMPORT_FILE_PATH))))
    });

    // declarations
    for (name, src_str) in [
        ("dec_int", SRC_DEC_INT),
        ("const_int", SRC_CONST_INT),
        ("dec_float", SRC_DEC_FLOAT),
        ("const_float", SRC_CONST_FLOAT),
        ("dec_string", SRC_DEC_STRING),
        ("const_string", SRC_CONST_STRING),
        ("dec_char", SRC_DEC_CHAR),
        ("const_char", SRC_CONST_CHAR),
        ("dec_bool", SRC_DEC_BOOL),
        ("const_bool", SRC_CONST_BOOL),
        ("dec_array", SRC_DEC_ARRAY),
        ("const_array", SRC_CONST_ARRAY),
        ("dec_fn", SRC_DEC_FN),
        ("const_fn", SRC_CONST_FN),
    ] {
        group.bench_with_input(BenchmarkId::new("declaration", name), src_str, |b, s| {
            b.iter(|| Tokenizer::lex(src("bench", black_box(s))))
        });
    }

    // while / if
    group.bench_function("while", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_WHILE))))
    });
    for (name, src_str) in [
        ("if_simple", SRC_IF_SIMPLE),
        ("if_else", SRC_IF_ELSE),
        ("if_else_if", SRC_IF_ELSE_IF),
        ("if_else_if_else", SRC_IF_ELSE_IF_ELSE),
        ("if_nested", SRC_IF_NESTED),
    ] {
        group.bench_with_input(BenchmarkId::new("conditional", name), src_str, |b, s| {
            b.iter(|| Tokenizer::lex(src("bench", black_box(s))))
        });
    }

    // functions / lambdas
    group.bench_function("fn_simple", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_FN_SIMPLE))))
    });
    group.bench_function("fn_fn_param", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_FN_FN_PARAM))))
    });
    group.bench_function("dec_fn_lambda", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_DEC_FN_LAMBDA))))
    });

    // programs
    group.bench_function("program/math_print", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_PROGRAM_MATH_PRINT))))
    });
    group.bench_function("program/geometry", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_PROGRAM_GEOMETRY))))
    });
    group.bench_function("program/fn_array", |b| {
        b.iter(|| Tokenizer::lex(src("bench", black_box(SRC_PROGRAM_FN_ARRAY))))
    });

    group.finish();
}

fn bench_v0_1_0_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("v0.1.0/parser");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(200);

    // for loops
    group.bench_function("for_c", |b| b.iter(|| lex_and_parse(SRC_FOR_C)));
    group.bench_function("for_range", |b| b.iter(|| lex_and_parse(SRC_FOR_RANGE)));
    group.bench_function("for_iterable", |b| {
        b.iter(|| lex_and_parse(SRC_FOR_ITERABLE))
    });

    // imports
    group.bench_function("import_simple", |b| {
        b.iter(|| lex_and_parse(SRC_IMPORT_SIMPLE))
    });
    group.bench_function("import_path", |b| b.iter(|| lex_and_parse(SRC_IMPORT_PATH)));
    group.bench_function("import_multi", |b| {
        b.iter(|| lex_and_parse(SRC_IMPORT_MULTI))
    });
    group.bench_function("import_multi_path", |b| {
        b.iter(|| lex_and_parse(SRC_IMPORT_MULTI_PATH))
    });
    group.bench_function("import_file", |b| b.iter(|| lex_and_parse(SRC_IMPORT_FILE)));
    group.bench_function("import_file_path", |b| {
        b.iter(|| lex_and_parse(SRC_IMPORT_FILE_PATH))
    });

    // declarations
    for (name, src_str) in [
        ("dec_int", SRC_DEC_INT),
        ("const_int", SRC_CONST_INT),
        ("dec_float", SRC_DEC_FLOAT),
        ("const_float", SRC_CONST_FLOAT),
        ("dec_string", SRC_DEC_STRING),
        ("const_string", SRC_CONST_STRING),
        ("dec_char", SRC_DEC_CHAR),
        ("const_char", SRC_CONST_CHAR),
        ("dec_bool", SRC_DEC_BOOL),
        ("const_bool", SRC_CONST_BOOL),
        ("dec_array", SRC_DEC_ARRAY),
        ("const_array", SRC_CONST_ARRAY),
        ("dec_fn", SRC_DEC_FN),
        ("const_fn", SRC_CONST_FN),
    ] {
        group.bench_with_input(BenchmarkId::new("declaration", name), src_str, |b, s| {
            b.iter(|| lex_and_parse(black_box(s)))
        });
    }

    // while / if
    group.bench_function("while", |b| b.iter(|| lex_and_parse(SRC_WHILE)));
    for (name, src_str) in [
        ("if_simple", SRC_IF_SIMPLE),
        ("if_else", SRC_IF_ELSE),
        ("if_else_if", SRC_IF_ELSE_IF),
        ("if_else_if_else", SRC_IF_ELSE_IF_ELSE),
        ("if_nested", SRC_IF_NESTED),
    ] {
        group.bench_with_input(BenchmarkId::new("conditional", name), src_str, |b, s| {
            b.iter(|| lex_and_parse(black_box(s)))
        });
    }

    // functions / lambdas
    group.bench_function("fn_simple", |b| b.iter(|| lex_and_parse(SRC_FN_SIMPLE)));
    group.bench_function("fn_fn_param", |b| b.iter(|| lex_and_parse(SRC_FN_FN_PARAM)));
    group.bench_function("dec_fn_lambda", |b| {
        b.iter(|| lex_and_parse(SRC_DEC_FN_LAMBDA))
    });

    // programs — also bench at repeat counts to observe scaling
    for size in [1, 10, 50] {
        let text = SRC_PROGRAM_MATH_PRINT.repeat(size);
        group.bench_with_input(
            BenchmarkId::new("program/math_print", size),
            &text,
            |b, s| b.iter(|| lex_and_parse(black_box(s))),
        );
        let text = SRC_PROGRAM_GEOMETRY.repeat(size);
        group.bench_with_input(BenchmarkId::new("program/geometry", size), &text, |b, s| {
            b.iter(|| lex_and_parse(black_box(s)))
        });
        let text = SRC_PROGRAM_FN_ARRAY.repeat(size);
        group.bench_with_input(BenchmarkId::new("program/fn_array", size), &text, |b, s| {
            b.iter(|| lex_and_parse(black_box(s)))
        });
    }

    group.finish();
}

fn bench_v0_1_0_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("v0.1.0/pipeline");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(200);

    group.bench_function("program/math_print", |b| {
        b.iter(|| lex_and_parse(black_box(SRC_PROGRAM_MATH_PRINT)))
    });
    group.bench_function("program/geometry", |b| {
        b.iter(|| lex_and_parse(black_box(SRC_PROGRAM_GEOMETRY)))
    });
    group.bench_function("program/fn_array", |b| {
        b.iter(|| lex_and_parse(black_box(SRC_PROGRAM_FN_ARRAY)))
    });

    group.finish();
}

criterion_group!(
    benches_v0_1_0,
    bench_v0_1_0_lexer,
    bench_v0_1_0_parser,
    bench_v0_1_0_pipeline
);
criterion_main!(benches_v0_1_0);
