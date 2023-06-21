use jayce::{regexify, Tokenizer, TokenizerResult};
use lazy_static::lazy_static;
use regex::Regex;

// Your token kind names and their regexes
lazy_static! {
    static ref DUOS: Vec<(&'static str, Regex)> = vec![
        ("price", regexify!(r"^[0-9]+\$")),
        ("operator", regexify!(r"^=")),
        ("name", regexify!(r"^[a-zA-Z_]+")),
    ];
}
// Source to tokenize
const SOURCE: &str = "Excalibur = 5000$";

fn main() {
    let mut jayce = Tokenizer::new(SOURCE, &DUOS);

    // Print all tokens until the end of source
    loop {
        match jayce.next() {
            TokenizerResult::Found(token) => println!("{:?}", token),
            TokenizerResult::End => break,
            TokenizerResult::Error(line, column) => {
                panic!("No match line {}, column {}.", line, column)
            }
        }
    }
}
