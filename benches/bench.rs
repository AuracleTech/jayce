#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{internal::DUOS_RUST, Tokenizer, TokenizerResult};

const SOURCE: &str = include_str!("../data/vulkan_triangle.rs");

fn jayce_initialization() -> Tokenizer<'static> {
    Tokenizer::new(SOURCE, &DUOS_RUST)
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
    c.bench_function("jayce_initialization", |b| {
        b.iter(|| jayce_initialization())
    });

    let mut tokenizer = Tokenizer::new(SOURCE, &DUOS_RUST);
    c.bench_function("jayce_tokenize", |b| {
        b.iter(|| jayce_tokenize(&mut tokenizer))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
