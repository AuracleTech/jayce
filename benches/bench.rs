#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::{Tokenizer, TokenizerResult};

fn jayce_initialization() -> Tokenizer<'static> {
    let source = include_str!("../src/lib.rs");
    let duos = [
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("string", r#"^"[^"]*""#),
        ("char", r"^'[^']*'"),
        ("keyword", r"^(let|if|else|fn|struct|enum|match)"),
        ("assign", r"^="),
        ("identifier", r"^[a-zA-Z_][a-zA-Z0-9_]*"),
        ("integer", r"^\d+"),
        ("float", r"^\d+\.\d+"),
        ("comment", r"^//.*"),
        ("block_comment", r"^/\*(.|\n)*?\*/"),
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
        ("plus", r"^\+"),
        ("minus", r"^-"),
        ("star", r"^\*"),
        ("slash", r"^/"),
        ("percent", r"^%"),
        ("pipe", r"^\|"),
        ("open_angle", r"^<"),
        ("close_angle", r"^>"),
        ("caret", r"^\^"),
        ("temp_borrow", r"^&"),
        ("temp_mut_borrow", r"^&mut"),
        ("question", r"^\?"),
        ("macro_exclamation", r"^!"),
    ];
    Tokenizer::new(source, &duos)
}

fn jayce_tokenize(jayce: &mut Tokenizer) {
    loop {
        match jayce.next() {
            TokenizerResult::Found(_) => {}
            TokenizerResult::End => break,
            TokenizerResult::Unknown(error) => {
                panic!("{:?}", error);
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("jayce_initialization", |b| {
        b.iter(|| jayce_initialization())
    });

    let mut tokenizer = jayce_initialization();
    c.bench_function("jayce_tokenize", |b| {
        b.iter(|| jayce_tokenize(black_box(&mut tokenizer)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
