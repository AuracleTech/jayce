#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{Tokenizer, TokenizerResult};

const RUST_DUOS: [(&str, &str); 28] = [
    (
        "keyword",
        r"^(let|if|else|fn|struct|enum|match|use|mod|pub|crate|impl|trait|for|while|loop|break|continue|return|as|const|static|type|where|unsafe|extern|ref|self|super|in|move|dyn|abstract|async|await|become|box|do|final|macro|override|priv|typeof|unsized|virtual|yield)",
    ),
    ("string", r#"^"[^"]*""#),
    ("char", r"^'(.|\\n)'"),
    ("lifetime", r"^'(?:[a-z_][a-z0-9_]*|static)"),
    ("operator", r"^(=|\+|-|\*|/|%)"),
    ("identifier", r"^[a-zA-Z_][a-zA-Z0-9_]*"),
    ("integer", r"^\d+"),
    ("float", r"^\d+\.\d+"),
    ("double_colon", r"^::"),
    ("semicolon", r"^;"),
    ("open_brace", r"^\{"),
    ("close_brace", r"^\}"),
    ("open_paren", r"^\("),
    ("close_paren", r"^\)"),
    ("open_bracket", r"^\["),
    ("close_bracket", r"^\]"),
    ("comma", r"^,"),
    ("hash", r"^#"),
    ("dot", r"^\."),
    ("colon", r"^:"),
    ("pipe", r"^\|"),
    ("open_angle", r"^<"),
    ("close_angle", r"^>"),
    ("caret", r"^\^"),
    ("temp_borrow", r"^&"),
    ("temp_mut_borrow", r"^&mut"),
    ("question", r"^\?"),
    ("macro_exclamation", r"^!"),
];

fn jayce_initialization(source: &str) -> Tokenizer {
    Tokenizer::new(source, RUST_DUOS.into())
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
