use jayce::{Tokenizer, TokenizerResult};

fn main() {
    let source = "Excalibur = 5000$";
    let duos = vec![
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("name", r"^[a-zA-Z_]+"),
        ("price", r"^[0-9]+\$"),
        ("equals", r"^="),
    ];
    let mut jayce = Tokenizer::new(source, &duos);

    while let TokenizerResult::Found(token) = jayce.next() {
        println!("{:?}", token);
    }
}
