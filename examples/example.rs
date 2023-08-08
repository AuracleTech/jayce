use jayce::{duos, Tokenizer};
use regex::Regex;

const SOURCE: &str = "Excalibur = 5000$; // Your custom lang";

lazy_static::lazy_static! (
    static ref DUOS: Vec<(&'static str, Regex)> = duos![
        "price", r"^[0-9]+\$",
        "semicolon", r"^;",
        "operator", r"^=",
        "name", r"^[a-zA-Z_]+"
    ];
);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::new(SOURCE, &DUOS);

    while let Some(token) = tokenizer.next()? {
        println!("{:?}", token);
    }

    Ok(())
}
