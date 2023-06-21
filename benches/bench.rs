#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{internal::DUOS_RUST, Tokenizer, TokenizerResult};

fn jayce_initialization(source: &str) -> Tokenizer {
    Tokenizer::new(source, &DUOS_RUST)
}

fn jayce_tokenize(jayce: &mut Tokenizer) {
    loop {
        match jayce.next() {
            TokenizerResult::Found(_) => continue,
            TokenizerResult::End => break,
            TokenizerResult::Error(line, column) => {
                panic!("Error at line {}, column {}.", line, column)
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let source = include_str!("../data/vulkan_triangle.rs");

    c.bench_function("jayce_initialization", |b| {
        b.iter(|| jayce_initialization(black_box(source)))
    });

    let mut tokenizer = jayce_initialization(source);
    c.bench_function("jayce_tokenize", |b| {
        b.iter(|| jayce_tokenize(&mut tokenizer))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
