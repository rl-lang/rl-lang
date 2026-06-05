use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rl_lang::lexer::tokenizer::Tokenizer;
use rl_lang::parser::parser_logic::Parser;

// source snippets
const SRC_ARRAY_DECL: &str = "\
dec arr[int] my_int_array = [10, 20, 30]
dec arr[bool] my_bool_array = [true, false, true]
dec arr[string] my_string_array = [\"my\", \"world\", \"hello\"]
dec arr[float] my_float_array = [1.0, 2.0, 3.0]
dec arr[char] my_char_array = ['.', 'r', 'l']
";

const SRC_VAR_DECL: &str = "\
dec bool my_bool = true
dec int my_int = 1
dec string my_string = \"string\"
dec float my_float = 1.0
dec char my_char = 'x'
";

const SRC_LOOP: &str = "\
get mod, pow from std::math
dec int i = 0
dec float x = 1.5
dec arr[float] arr_x = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
while (i < 10) {
  if ( mod(x, (x / 2.0)) > 10.0 ) {
    arr_x[i] = x + pow(x, x)
  } else if ( mod(x, (x / 3.0)) == 0.0) {
    arr_x[i] = x + -x * (x + pow(x , 3))
  } else {
    arr_x[i] = 90.09
  }
  x += x + 12.4
  i += 1
}
";

const SRC_FULL: &str = "\
get mod, pow, sin, cos, tan from std::math
dec arr[int] my_int_array = [10, 20, 30]
dec arr[bool] my_bool_array = [true, false, true]
dec arr[string] my_string_array = [\"my\", \"world\", \"hello\"]
dec arr[float] my_float_array = [1.0, 2.0, 3.0]
dec arr[char] my_char_array = ['.', 'r', 'l']
dec bool my_bool = true
dec int my_int = 1
dec string my_string = \"string\"
dec float my_float = 1.0
dec char my_char = 'x'
my_bool = !my_bool
my_bool_array[0] = my_bool
my_int += 3
my_int += pow(my_int,my_int)
my_int_array[0] = mod(my_int_array[1], my_int_array[2])
dec float my_float_sin = sin(my_float)
dec float my_int_cos = cos(my_int)
dec float my_float_tan = tan(my_float)
dec int i = 0
dec float x = 1.5
dec arr[float] arr_x = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
while (i < 10) {
  if ( mod(x, (x / 2.0)) > 10.0 ) {
    arr_x[i] = x + pow(x, x)
  } else if ( mod(x, (x / 3.0)) == 0.0) {
    arr_x[i] = x + -x * (x + pow(x , 3))
  } else {
    arr_x[i] = 90.09
  }
  x += x + 12.4
  i += 1
}
";

// -=lexer benchmarks=-

fn bench_lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer");
    group.measurement_time(std::time::Duration::from_secs(11));
    group.sample_size(200);

    for size in [1, 100, 500] {
        let src = SRC_ARRAY_DECL.repeat(size);
        group.bench_with_input(
            BenchmarkId::new("array declarations", size),
            &src,
            |b, s| b.iter(|| Tokenizer::lex(black_box(s))),
        );

        let src = SRC_VAR_DECL.repeat(size);
        group.bench_with_input(BenchmarkId::new("var declarations", size), &src, |b, s| {
            b.iter(|| Tokenizer::lex(black_box(s)))
        });

        let src = SRC_LOOP.repeat(size);
        group.bench_with_input(BenchmarkId::new("while loop", size), &src, |b, s| {
            b.iter(|| Tokenizer::lex(black_box(s)))
        });
    }

    // full program — only makes sense at size 1
    group.bench_function("full program", |b| {
        b.iter(|| Tokenizer::lex(black_box(SRC_FULL)))
    });

    group.finish();
}

// -=parser benchmarks=-

fn bench_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    group.measurement_time(std::time::Duration::from_secs(11));
    group.sample_size(200);

    for size in [1, 100, 500] {
        let src = SRC_ARRAY_DECL.repeat(size);
        group.bench_with_input(
            BenchmarkId::new("array declarations", size),
            &src,
            |b, s| b.iter(|| Parser::parse(Tokenizer::lex(black_box(s)))),
        );

        let src = SRC_VAR_DECL.repeat(size);
        group.bench_with_input(BenchmarkId::new("var declarations", size), &src, |b, s| {
            b.iter(|| Parser::parse(Tokenizer::lex(black_box(s))))
        });

        let src = SRC_LOOP.repeat(size);
        group.bench_with_input(BenchmarkId::new("while loop", size), &src, |b, s| {
            b.iter(|| Parser::parse(Tokenizer::lex(black_box(s))))
        });
    }

    group.bench_function("full program", |b| {
        b.iter(|| Parser::parse(Tokenizer::lex(black_box(SRC_FULL))))
    });

    group.finish();
}

// -=lexer + parser benchmarks=-

fn bench_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline");
    group.measurement_time(std::time::Duration::from_secs(11));
    group.sample_size(200);

    group.bench_function("full program", |b| {
        b.iter(|| {
            let tokens = Tokenizer::lex(black_box(SRC_FULL));
            Parser::parse(tokens)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_lexer, bench_parser, bench_pipeline);
criterion_main!(benches);
