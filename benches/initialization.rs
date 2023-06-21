#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{internal::DUOS_RUST, Tokenizer};

const SOURCE: &str = include_str!("../data/vulkan_triangle.rs");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("initialization", |b| {
        b.iter(|| Tokenizer::new(SOURCE, &DUOS_RUST))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
