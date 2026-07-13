use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rl_benches::*;

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

criterion_group!(benches_v0_1_4_lexer, bench_v0_1_4_lexer);
criterion_main!(benches_v0_1_4_lexer);
