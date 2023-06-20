fn main() {
    use jayce::{Tokenizer, TokenizerResult};

    let source = "Excalibur = 5000$";
    let duos = vec![
        ("price", r"^[0-9]+\$"),
        ("operator", r"^="),
        ("name", r"^[a-zA-Z_]+"),
    ];
    let mut jayce = Tokenizer::new(source, duos);

    loop {
        match jayce.next() {
            TokenizerResult::Found(token) => println!("{:?}", token),
            TokenizerResult::End => break,
            TokenizerResult::Error(line, column) => {
                panic!("Error line {}, column {}.", line, column)
            }
        }
    }
}
