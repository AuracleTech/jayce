use jayce::{Duo, SeekResult};

const SOURCE: &str = "Excalibur = 5000$; // Your own language!";

lazy_static::lazy_static! {
    static ref DUOS: Vec<Duo<&'static str>> = vec![
        Duo::new("whitespace", r"^[^\S\n]+", false),
        Duo::new("commentLine", r"^//(.*)", false),
        Duo::new("commentBlock", r"^/\*(.|\n)*?\*/", false),
        Duo::new("newline", r"^\n", false),

        Duo::new("price", r"^[0-9]+\$", true),
        Duo::new("semicolon", r"^;", true),
        Duo::new("operator", r"^=", true),
        Duo::new("name", r"^[a-zA-Z_]+", true)
    ];
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokenizer = jayce::Tokenizer::new(SOURCE, &DUOS);

    while let Ok(tokenize_result) = tokenizer.seek() {
        match tokenize_result {
            SeekResult::Token(token) => println!("{:?}", token),
            SeekResult::Skipped => continue,
            SeekResult::End => break,
        }
    }

    Ok(())
}
