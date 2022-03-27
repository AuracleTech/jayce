#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
use regex::Regex;

mod token;
pub use token::Token;

pub struct Jayce<'a> {
    source: &'a str,
    duos: Vec<(&'a str, Regex)>,
    cursor: usize,
    line: u32,
    column: u32,
}

impl<'a> Jayce<'a> {
    pub fn new(source: &'a str, duos: &[(&'a str, &str)]) -> Jayce<'a> {
        Self {
            source,
            duos: duos
                .iter()
                .map(|&(k, v)| (k, Regex::new(v).expect("Invalid regex.")))
                .collect(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }
    /*
    TODO : token lifetime on token.kind and token.value.
    duos
                .into_iter()
                .map(|(s1, s2)| {
                    (
                        s1.to_owned(),
                        Regex::new(&s2).expect("Failed to parse regex from string."),
                    )
                })
                .collect(),
    */

    pub fn eat(&mut self) -> Token {
        if self.cursor >= self.source.len() {
            return Token::from("EoF", "End of File", self.line, self.column);
        }

        while self.source[self.cursor..].starts_with('\n') {
            self.line += 1;
            self.cursor += 1;
            self.column = 1;
        }

        let buffer = &self.source[self.cursor..];

        for duo in self.duos.iter() {
            let result = &duo.1.find(buffer);
            if let Some(result) = result {
                self.cursor += result.end();
                self.column += result.end() as u32;
                return Token {
                    kind: duo.0,
                    value: result.as_str(),
                    line: self.line,
                    column: self.column,
                };
            }
        }
        panic!(
            "No regex match found on line {} column {}",
            self.line, self.column
        );
    }
}
