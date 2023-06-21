#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{internal::DUOS_RUST, Tokenizer, TokenizerResult};

const SOURCE: &str = include_str!("../data/vulkan_triangle.rs");

fn criterion_benchmark(c: &mut Criterion) {
    let mut jayce = Tokenizer::new(SOURCE, &DUOS_RUST);
    c.bench_function("tokenize", |b| {
        b.iter(|| loop {
            match jayce.next() {
                TokenizerResult::Found(_) => continue,
                TokenizerResult::End => break,
                TokenizerResult::Error(line, column) => {
                    panic!("Error at line {}, column {}.", line, column)
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
