use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pcisph_rs;

fn dam_break(n: usize, i: usize) {
    let mut state = pcisph_rs::State::new();
    state.init_dam_break(n);
    for _ in 0..i {
        state.update();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-10");
    group.sample_size(10);
    group.bench_function("dam break: n=5000, i=100", |b| {
        b.iter(|| dam_break(black_box(5000), black_box(100)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
