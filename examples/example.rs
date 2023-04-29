use jayce::Tokenizer;

fn main() {
    let source = "let value = 5000$";
    let duos = &[
        ("keywords", r"^(let|const)"),
        ("whitespace", r"^\s+"),
        ("variable", r"^[a-zA-Z_]+"),
        ("price", r"^[0-9]+\$"),
        ("equals", r"^="),
        ("newline", r"^\n"),
    ];
    let mut tokenizer = Tokenizer::new(source, duos);

    while let Some(token) = tokenizer.eat() {
        println!("{:?}", token);
    }
}
