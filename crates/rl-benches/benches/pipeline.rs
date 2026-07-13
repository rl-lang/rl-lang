use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rl_benches::*;

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

criterion_group!(benches_v0_1_4_pipeline, bench_v0_1_4_pipeline);
criterion_main!(benches_v0_1_4_pipeline);
