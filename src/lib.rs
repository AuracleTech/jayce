#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
use regex::Regex;

mod token;
pub use token::Token;

pub struct Jayce<'a> {
    pub source: &'a str,
    pub duos: Vec<(&'a str, Regex)>,
    pub cursor: usize,
    pub line: u32,
    pub column: u32,
    pub eat_count: usize,
    pub tokens: Vec<Token<'a>>,
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
            eat_count: 0,
            tokens: Vec::new(),
        }
    }

    pub fn eat(&mut self) -> Option<Token<'a>> {
        if self.cursor >= self.source.len() {
            return None;
        }

        let buffer = &self.source[self.cursor..];
        let mut kind = "unknown";
        let mut value = &buffer[0..1];

        for duo in self.duos.iter() {
            if let Some(result) = &duo.1.find(buffer) {
                kind = duo.0;
                value = result.as_str();
                let newlines = value.matches('\n').count();
                if newlines > 0 {
                    self.line += newlines as u32;
                    self.column = 0;
                }
                break;
            }
        }

        self.cursor += value.len();
        self.column += value.len() as u32;
        self.eat_count += 1;

        let token = Token::from(kind, value, self.line, self.column);
        self.tokens.push(token.clone());
        Some(token)
    }
}
