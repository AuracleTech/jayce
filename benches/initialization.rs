#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{internal::duos_rust, Tokenizer};

const EMPTY_SOURCE: &str = "";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("initialization", |b| {
        b.iter(|| black_box(Tokenizer::new(EMPTY_SOURCE, duos_rust())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
