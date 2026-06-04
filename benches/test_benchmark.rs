use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_lexer(c: &mut Criterion) {
    let source = "dec int x = 10\ndec int y = 20\n".repeat(100);
    c.bench_function("lexer 100 declarations", |b| {
        b.iter(|| rl_lang::lexer::tokenizer::Tokenizer::lex(black_box(&source)))
    });
}

criterion_group!(benches, bench_lexer);
criterion_main!(benches);
