#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jayce::Tokenizer;

fn tokenize() {
    let mut jayce = Tokenizer::new();
    jayce.add("newline", r"^\n");
    jayce.add("whitespace", r"^\s+");
    jayce.add("string", r#"^"[^"]*""#);
    jayce.add("char", r"^'[^']*'");
    jayce.add("keyword", r"^(let|if|else|fn|struct|enum|match)");
    jayce.add("assign", r"^=");
    jayce.add("identifier", r"^[a-zA-Z_][a-zA-Z0-9_]*");
    jayce.add("integer", r"^\d+");
    jayce.add("float", r"^\d+\.\d+");
    jayce.add("comment", r"^//.*");
    jayce.add("block_comment", r"^/\*(.|\n)*?\*/");
    jayce.add("double_colon", r"^::");
    jayce.add("semicolon", r"^;");
    jayce.add("open_brace", r"^\{");
    jayce.add("close_brace", r"^\}");
    jayce.add("open_paren", r"^\(");
    jayce.add("close_paren", r"^\)");
    jayce.add("open_bracket", r"^\[");
    jayce.add("close_bracket", r"^\]");
    jayce.add("comma", r"^,");
    jayce.add("hash", r"^#");
    jayce.add("dot", r"^\.");
    jayce.add("colon", r"^:");
    jayce.add("plus", r"^\+");
    jayce.add("minus", r"^-");
    jayce.add("star", r"^\*");
    jayce.add("slash", r"^/");
    jayce.add("percent", r"^%");
    jayce.add("pipe", r"^\|");
    jayce.add("open_angle", r"^<");
    jayce.add("close_angle", r"^>");
    jayce.add("caret", r"^\^");
    jayce.add("temp_borrow", r"^&");
    jayce.add("temp_mut_borrow", r"^&mut");
    jayce.add("question", r"^\?");
    jayce.add("macro_exclamation", r"^!");

    let source = include_str!("../src/lib.rs");

    loop {
        match jayce.next(source) {
            jayce::TokenizerResult::Token(_) => {}
            jayce::TokenizerResult::End => break,
            jayce::TokenizerResult::Nothing(error) => {
                println!("{:?}", error);
                break;
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tokenize", |b| b.iter(|| tokenize()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
