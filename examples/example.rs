use jayce::{Duo, Tokenizer};
use std::sync::OnceLock;

const SOURCE: &str = "Excalibur = 5000$; // Your own language!";

fn duos() -> &'static Vec<Duo<&'static str>> {
    static DUOS: OnceLock<Vec<Duo<&'static str>>> = OnceLock::new();
    DUOS.get_or_init(|| {
        vec![
            Duo::new("whitespace", r"^[^\S\n]+", false),
            Duo::new("commentLine", r"^//(.*)", false),
            Duo::new("commentBlock", r"^/\*(.|\n)*?\*/", false),
            Duo::new("newline", r"^\n", false),
            Duo::new("price", r"^[0-9]+\$", true),
            Duo::new("semicolon", r"^;", true),
            Duo::new("operator", r"^=", true),
            Duo::new("name", r"^[a-zA-Z_]+", true),
        ]
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::new(SOURCE, duos());

    while let Some(token) = tokenizer.consume()? {
        println!("{:?}", token);
    }

    Ok(())
}
