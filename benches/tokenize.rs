#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{internal::DUOS_RUST, Tokenizer};

fn criterion_benchmark(c: &mut Criterion) {
    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory")
        .join("data");

    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().unwrap_or_default() == "rs" {
                    files.push((
                        file_path.file_name().unwrap().to_str().unwrap().to_owned(),
                        std::fs::read_to_string(&file_path).expect("Unable to read file"),
                    ));
                }
            }
        }
    }

    let mut total = 0;

    c.bench_function(
        "Tokenize Yuumi vulkan game engine",
        |b: &mut criterion::Bencher<'_>| {
            b.iter(|| {
                total = 0;

                for (_, source) in files.iter() {
                    let mut tokenizer = Tokenizer::new(black_box(&source), black_box(&DUOS_RUST));

                    let subtotal_tokens = match tokenizer.tokenize_all() {
                        Ok(tokens) => tokens.len(),
                        Err(err) => panic!("{}", err),
                    };

                    total += subtotal_tokens;
                }
            })
        },
    );

    println!("Amount of tokens created : {}", total);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
