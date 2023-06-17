use jayce::{Tokenizer, TokenizerResult};

fn main() {
    let mut jayce = Tokenizer::new();
    jayce.add("newline", r"^\n");
    jayce.add("whitespace", r"^\s+");
    jayce.add("name", r"^[a-zA-Z_]+");
    jayce.add("price", r"^[0-9]+\$");
    jayce.add("equals", r"^=");

    let source = "Excalibur = 5000$";

    while let TokenizerResult::Token(token) = jayce.next(source) {
        println!("{:?}", token);
    }
}
